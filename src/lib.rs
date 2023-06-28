mod data;
mod helpers;
mod hero;
mod map;
mod npc;
mod pizza_stone;
mod camera;
mod item;

use crate::map::Map;
use helpers::{get_direction, DirectionInput, Event, Direction};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen(module = "/web/exports.js")]
extern "C" {
    fn emit_event(name: &str, who: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        log(&format_args!($($t)*).to_string())
    };
}

#[wasm_bindgen]
pub struct OverWorld {
    map: Map,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    dir_input: DirectionInput,
    story_flags: Vec<String>
}

#[wasm_bindgen]
impl OverWorld {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        Self {
            dir_input: DirectionInput::new(),
            ctx: canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap(),
            canvas,
            map: Map::new(0, Direction::Down),
            story_flags: vec![String::from("START")],
        }
    }

    pub fn update(&mut self) {
        if !self.map.is_cutscene_playing {
            self.map.update_npcs();
            self.map.update_hero(&self.dir_input.held_directions);
        } else {
            self.map.update_cutscene();
        }
    }

    pub fn draw(&mut self) {
        self.ctx.clear_rect(
            0f64,
            0f64,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
        self.map.draw(&self.ctx, "lower");
        self.map.draw_characters(&self.ctx);
        self.map.draw(&self.ctx, "upper");
    }

    pub fn add_direction(&mut self, direction_number: usize) {
        self.dir_input.add(direction_number);
    }

    pub fn remove_direction(&mut self, direction_number: usize) {
        self.dir_input.remove(direction_number);
    }

    pub fn is_hero_movable(&self) -> bool {
        self.map.hero.movement.moveable
    }

    pub fn add_story_flag(&mut self, story_flag: String) {
        if !self.story_flags.contains(&story_flag) {
            self.story_flags.push(story_flag);
        }
    }

    pub fn remove_story_flag(&mut self, story_flag: String) {
        if let Some(index) = self.story_flags.iter().position(|s| *s == story_flag) {
            self.story_flags.swap_remove(index);
        };
    }
}

// cutscene
#[wasm_bindgen]
impl OverWorld {
    pub fn start_cutscene(&mut self) {
        self.map.is_cutscene_playing = true;
    }

    pub fn stop_cutscene(&mut self) {
        self.map.is_cutscene_playing = false;

        self.map
            .npcs
            .iter_mut()
            .for_each(|character| character.cutscene = None);
        self.map.hero.cutscene = None;
        self.map.hero.movement.progress_remaining = 0;
    }

    pub fn walk(&mut self, who: &str, direction: &str) {
        match who {
            "hero" => {
                self.map.hero.cutscene = Some((Event::Walk, get_direction(direction)));
                self.map.hero.movement.progress_remaining = 16;
            }
            x => self
                .map
                .set_npc_cutscene(x, get_direction(direction), Event::Walk),
        }
    }

    pub fn stand(&mut self, who: &str, direction: &str) {
        match who {
            "hero" => {
                self.map.hero.cutscene = Some((Event::Stand, get_direction(direction)));
                self.map.hero.movement.progress_remaining = 16;
            }
            x => self
                .map
                .set_npc_cutscene(x, get_direction(direction), Event::Stand),
        }
    }

    pub fn check_for_action_cutscene(&mut self) -> JsValue {
        match self.map.check_for_action_cutscene(&self.story_flags) {
            Some(x) => serde_wasm_bindgen::to_value(x).unwrap(),
            None => serde_wasm_bindgen::to_value(&false).unwrap(),
        }
    }

    pub fn check_for_action(&self) -> JsValue {
        match self.map.check_for_action_square(&self.story_flags) {
            Some(x) => serde_wasm_bindgen::to_value(x).unwrap(),
            None => serde_wasm_bindgen::to_value(&false).unwrap(),
        }
    }

    pub fn is_cutscene_playing(&self) -> bool {
        self.map.is_cutscene_playing
    }

    pub fn set_camera(&mut self, value: JsValue) {
        let location: [String; 2] = serde_wasm_bindgen::from_value(value).unwrap();
        let location: Vec<f64> = location.iter().map(|s| s.parse::<f64>().unwrap()).collect();

        self.map.camera.set_motion(location.try_into().unwrap());
    }
}


// load progress
#[wasm_bindgen]
impl OverWorld {
    pub fn change_map(&mut self, index: &str, direction: &str) {
        self.map = Map::new(index.parse::<usize>().unwrap(), get_direction(direction));

        self.map.pizza_stones.iter_mut().for_each(|stone| {
            if self.story_flags.contains(&stone.flag) {
                stone.used = true;
            }
        });

        self.map.items.retain(|item| !self.story_flags.contains(&item.flag));
    }

    pub fn get_map_id(&self) -> String {
        self.map.index.to_string()
    }

    pub fn get_story_flags(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.story_flags).unwrap()
    }

    pub fn set_story_flags_from_progress(&mut self, object: JsValue) {
        let flags = serde_wasm_bindgen::from_value(object).unwrap();

        self.story_flags = flags;
    }

    pub fn set_pizza_stone_used(&mut self, index: &str) {
        self.map.pizza_stones[index.parse::<usize>().unwrap()].used = true;
    }

    pub fn remove_item(&mut self, index: &str) {
        self.map.items.remove(index.parse::<usize>().unwrap());
    }
}