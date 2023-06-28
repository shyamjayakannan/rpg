use crate::{
    data::DATA,
    helpers::{get_next_place, get_opposite_direction, Direction, Event, Image},
    hero::Hero,
    npc::Npc, pizza_stone::PizzaStone, camera::Camera, item::Item,
};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::CanvasRenderingContext2d;

pub struct Map {
    pub index: usize,
    upper_image: Image,
    lower_image: Image,
    pub walls: Vec<[u8; 2]>,
    pub is_cutscene_playing: bool,
    pub npcs: Vec<Npc>,
    pub hero: Hero,
    pub pizza_stones: Vec<PizzaStone>,
    pub items: Vec<Item>,
    pub camera: Camera,
}

impl Map {
    pub fn new(index: usize, hero_direction: Direction) -> Self {
        Self {
            upper_image: Image::new(DATA[index].1),
            lower_image: Image::new(DATA[index].0),
            is_cutscene_playing: false,
            walls: Vec::from(DATA[index].2)
                .iter()
                .map(|pair| {
                    pair.iter()
                        .map(|value| value * 16)
                        .collect::<Vec<u8>>()
                        .try_into()
                        .unwrap()
                })
                .collect(),
            npcs: (0..DATA[index].4.len())
                .map(|id| Npc::new(index, id))
                .collect(),
            pizza_stones: (0..DATA[index].8.len())
                .map(|id| PizzaStone::new(index, id))
                .collect(),
            items: (0..DATA[index].9.len())
                .map(|id| Item::new(index, id))
                .collect(),
            index,
            hero: Hero::new(
                "./images/characters/people/hero.png",
                DATA[index].3[0],
                DATA[index].3[1],
                hero_direction,
            ),
            camera: Camera::new(DATA[index].3[0], DATA[index].3[1]),
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
        self.hero
            .draw(ctx, [&self.camera.dx, &self.camera.dy]);
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

    pub fn set_npc_cutscene(&mut self, id: &str, direction: Direction, event: Event) {
        let npc = &mut self.npcs[id.parse::<usize>().unwrap()];

        npc.cutscene = Some((event, direction));
        npc.movement.progress_remaining = 16;
    }

    pub fn check_for_action_cutscene(
        &mut self,
        story_flags: &Vec<String>,
    ) -> Option<&[&[[&str; 2]]]> {
        let next = get_next_place(&self.hero.direction, &self.hero.dx, &self.hero.dy);

        match self.npcs.iter().position(|npc| next[0] == npc.dx as u8 && next[1] == npc.dy as u8) {
            Some(x) => {
                self.npcs.get_mut(x).unwrap().direction =
                    get_opposite_direction(&self.hero.direction);

                match DATA[self.index].5[x].iter().find(|tuple| {
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
            },
            None => {
                match self.pizza_stones.iter().find(|pizza_stone| next[0] == pizza_stone.dx as u8 && next[1] == pizza_stone.dy as u8) {
                    Some(x) => match x.used {
                        true => Some(&PizzaStone::TALK_USED),
                        false => Some(&x.craft_event),
                    },
                    None => {
                        match self.items.iter().find(|item| next[0] == item.dx as u8 && next[1] == item.dy as u8) {
                            Some(x) => Some(&x.item_event),
                            None => None,
                        }
                    },
                }
            },
        }
    }

    pub fn check_for_action_square(&self, story_flags: &Vec<String>) -> Option<&[&[[&str; 2]]]> {
        match DATA[self.index].6.iter().position(|square| {
            (&self.hero.dx / 16.0) as u8 == square[0] && (&self.hero.dy / 16.0) as u8 == square[1]
        }) {
            Some(x) => {
                match DATA[self.index].7[x].iter().find(|tuple| {
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
