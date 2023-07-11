use crate::{
    camera::Camera,
    helpers::{get_next_place, get_opposite_direction, Direction, Event, Image, NpcCutscenes, ActionCutscenes},
    hero::Hero,
    item::{Item, ItemData},
    npc::{Npc, NpcData},
    pizza_stone::{PizzaStone, PizzaStoneData}
};
use serde::{Serialize, Deserialize};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapData {
    pub name: String,
    pub background_color: String,
    pub upper_image: String,
    pub lower_image: String,
    pub walls: Vec<[u16; 2]>,
    pub npcs: Vec<NpcData>,
    pub hero_position: [u16; 2],
    pub pizza_stones: Vec<PizzaStoneData>,
    pub items: Vec<ItemData>,
    pub npc_cutscenes: Vec<NpcCutscenes>,
    pub action_cutscenes: Vec<ActionCutscenes>,
}

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
    pub npc_cutscenes: Vec<NpcCutscenes>,
    pub action_cutscenes: Vec<ActionCutscenes>,
}

impl Map {
    pub fn new(
        data: MapData,
        hero_direction: Direction,
        hero_position: [u16; 2],
        canvas: &HtmlCanvasElement,
    ) -> Self {
        let mut walls: Vec<[u16; 2]> = data.walls
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

        canvas.style().set_property("background", &data.background_color).unwrap();

        Self {
            name: data.name,
            upper_image: Image::new(&data.upper_image),
            lower_image: Image::new(&data.lower_image),
            is_cutscene_playing: false,
            walls,
            npcs: data.npcs.into_iter().map(|npc| Npc::new(npc)).collect(),
            pizza_stones: data.pizza_stones
                .into_iter()
                .map(|pizza_stone| PizzaStone::new(pizza_stone))
                .collect(),
            items: data.items
                .into_iter()
                .map(|item| Item::new(item))
                .collect(),
            hero: Hero::new(
                "images/characters/people/hero.png",
                hero_position[0] as f64 * 16.0,
                hero_position[1] as f64 * 16.0,
                hero_direction,
            ),
            camera: Camera::new(hero_position[0] as f64 * 16.0, hero_position[1] as f64 * 16.0),
            npc_cutscenes: data.npc_cutscenes,
            action_cutscenes: data.action_cutscenes
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

    pub fn check_for_action_cutscene(
        &mut self,
        story_flags: &Vec<String>,
    ) -> Option<&Vec<Vec<[String; 2]>>> {
        let next = get_next_place(&self.hero.direction, &self.hero.dx, &self.hero.dy);

        match self
            .npcs
            .iter_mut()
            .find(|npc| next[0] == npc.dx as u16 && next[1] == npc.dy as u16)
        {
            Some(x) => {
                x.direction = get_opposite_direction(&self.hero.direction);

                match self
                    .npc_cutscenes
                    .iter()
                    .find(|cutscene| cutscene.name == x.name)
                {
                    Some(y) => {
                        match y.scenes.iter().find(|scene| {
                            for flag in &scene.flags {
                                if !story_flags.contains(flag) {
                                    return false;
                                }
                            }

                            true
                        }) {
                            Some(x) => Some(&x.scene),
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
                        true => Some(&self.pizza_stones[x].talk_used),
                        false => Some(&self.pizza_stones[x].scene.scene),
                    },
                    None => {
                        match self
                            .items
                            .iter()
                            .position(|item| next[0] == item.dx as u16 && next[1] == item.dy as u16)
                        {
                            Some(x) => Some(&self.items[x].scene.scene),
                            None => None,
                        }
                    }
                }
            }
        }
    }

    pub fn check_for_action_square(
        &self,
        story_flags: &Vec<String>,
    ) -> Option<&Vec<Vec<[String; 2]>>> {
        match self.action_cutscenes.iter().find(|square| {
            (&self.hero.dx / 16.0) as u16 == square.position[0]
                && (&self.hero.dy / 16.0) as u16 == square.position[1]
        }) {
            Some(x) => {
                match x.scenes.iter().find(|scene| {
                    for flag in &scene.flags {
                        if !story_flags.contains(flag) {
                            return false;
                        }
                    }

                    true
                }) {
                    Some(x) => Some(&x.scene),
                    None => None,
                }
            }
            None => None,
        }
    }
}
