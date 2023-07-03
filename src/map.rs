use crate::{
    camera::Camera,
    helpers::{get_next_place, get_opposite_direction, Direction, Event, Image},
    hero::Hero,
    item::Item,
    maps::Room,
    npc::Npc,
    pizza_stone::PizzaStone
};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Map {
    pub name: String,
    upper_image: Image,
    lower_image: Image,
    pub walls: Vec<[u16; 2]>,
    pub is_cutscene_playing: bool,
    pub npcs: Vec<Npc>,
    pub hero: Hero,
    pub pizza_stones: Vec<PizzaStone>,
    pub items: Vec<Item>,
    pub camera: Camera,
}

impl Map {
    pub fn new(
        name: &str,
        map: &Box<dyn Room>,
        hero_direction: Direction,
        hero_position: [u16; 2],
        canvas: &HtmlCanvasElement,
    ) -> Self {
        let npcs = map.npcs();
        let pizza_stones = map.pizza_stones();
        let items = map.items();
        let mut walls: Vec<[u16; 2]> = Vec::from(map.walls())
            .iter()
            .map(|pair| {
                pair.iter()
                    .map(|value| *value as u16 * 16)
                    .collect::<Vec<u16>>()
                    .try_into()
                    .unwrap()
            })
            .collect();

        walls.push([
            hero_position[0] as u16 * 16,
            hero_position[1] as u16 * 16,
        ]);

        canvas.style().set_property("background", map.background_color()).unwrap();

        Self {
            name: name.to_string(),
            upper_image: Image::new(map.upper_image()),
            lower_image: Image::new(map.lower_image()),
            is_cutscene_playing: false,
            walls,
            npcs: npcs.iter().map(|npc| Npc::new(npc)).collect(),
            pizza_stones: pizza_stones
                .iter()
                .map(|pizza_stone| PizzaStone::new(&(pizza_stone.1, pizza_stone.2), pizza_stone.0))
                .collect(),
            items: items
                .iter()
                .map(|item| Item::new(&(item.1, item.2), item.0))
                .collect(),
            hero: Hero::new(
                "images/characters/people/hero.png",
                hero_position[0] as f64 * 16.0,
                hero_position[1] as f64 * 16.0,
                hero_direction,
            ),
            camera: Camera::new(hero_position[0] as f64 * 16.0, hero_position[1] as f64 * 16.0),
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, which: &str) {
        let x;

        match which {
            "upper" => x = &self.upper_image,
            _ => x = &self.lower_image,
        }

        if x.loaded() {
            ctx.draw_image_with_html_image_element(
                x.image_ref(),
                10.5 * 16f64 - self.camera.dx,
                6f64 * 16f64 - self.camera.dy,
            )
            .unwrap_throw();
        }
    }

    pub fn draw_characters(&self, ctx: &CanvasRenderingContext2d) {
        self.npcs
            .iter()
            .for_each(|npc| npc.draw(ctx, [&self.camera.dx, &self.camera.dy]));
        self.pizza_stones
            .iter()
            .for_each(|pizza_stone| pizza_stone.draw(ctx, [&self.camera.dx, &self.camera.dy]));
        self.items
            .iter()
            .for_each(|item| item.draw(ctx, [&self.camera.dx, &self.camera.dy]));
        self.hero.draw(ctx, [&self.camera.dx, &self.camera.dy]);
    }

    pub fn update_npcs(&mut self) {
        // update camera
        self.camera.update([&self.hero.dx, &self.hero.dy]);

        self.npcs
            .iter_mut()
            .for_each(|npc| npc.update(&mut self.walls));
        self.pizza_stones
            .iter_mut()
            .for_each(|pizza_stone| pizza_stone.update());
    }

    pub fn update_hero(&mut self, direction_vec: &Vec<Direction>) {
        self.hero.update(direction_vec, &mut self.walls);
    }

    pub fn update_cutscene(&mut self) {
        self.camera.move_camera();
        self.npcs
            .iter_mut()
            .for_each(|npc| npc.update_cutscene(&mut self.walls));
        self.hero.update_cutscene(&mut self.walls);
    }

    pub fn set_npc_cutscene(&mut self, name: &str, direction: Direction, event: Event) {
        let npc = &mut self.npcs.iter_mut().find(|npc| npc.name == name).unwrap();

        npc.cutscene = Some((event, direction));
        npc.movement.progress_remaining = 16;
    }

    pub fn check_for_action_cutscene<'a, 'b: 'a>(
        &'a mut self,
        story_flags: &Vec<String>,
        map: &'b Box<dyn Room>,
    ) -> Option<&'b [&[[&str; 2]]]> {
        let next = get_next_place(&self.hero.direction, &self.hero.dx, &self.hero.dy);

        match self
            .npcs
            .iter_mut()
            .find(|npc| next[0] == npc.dx as u16 && next[1] == npc.dy as u16)
        {
            Some(x) => {
                x.direction = get_opposite_direction(&self.hero.direction);

                match map
                    .npc_cutscenes()
                    .iter()
                    .find(|cutscene| cutscene.0 == x.name)
                {
                    Some(y) => {
                        match y.1.iter().find(|tuple| {
                            for flag in tuple.0 {
                                if !story_flags.contains(&flag.to_string()) {
                                    return false;
                                }
                            }

                            true
                        }) {
                            Some(x) => Some(x.1),
                            None => None,
                        }
                    }
                    None => None,
                }
            }
            None => {
                match self.pizza_stones.iter().position(|pizza_stone| {
                    next[0] == pizza_stone.dx as u16 && next[1] == pizza_stone.dy as u16
                }) {
                    Some(x) => match self.pizza_stones[x].used {
                        true => Some(&PizzaStone::TALK_USED),
                        false => Some(map.pizza_stones()[x].3),
                    },
                    None => {
                        match self
                            .items
                            .iter()
                            .position(|item| next[0] == item.dx as u16 && next[1] == item.dy as u16)
                        {
                            Some(x) => Some(map.items()[x].3),
                            None => None,
                        }
                    }
                }
            }
        }
    }

    pub fn check_for_action_square<'a, 'b: 'a>(
        &'a self,
        story_flags: &Vec<String>,
        map: &'b Box<dyn Room>,
    ) -> Option<&'b [&[[&str; 2]]]> {
        match map.action_cutscenes().iter().find(|square| {
            (&self.hero.dx / 16.0) as u16 == square.0[0]
                && (&self.hero.dy / 16.0) as u16 == square.0[1]
        }) {
            Some(x) => {
                match x.1.iter().find(|tuple| {
                    for flag in tuple.0 {
                        if !story_flags.contains(&flag.to_string()) {
                            return false;
                        }
                    }

                    true
                }) {
                    Some(x) => Some(x.1),
                    None => None,
                }
            }
            None => None,
        }
    }
}
