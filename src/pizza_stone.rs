use serde::{Serialize, Deserialize};
use web_sys::CanvasRenderingContext2d;

use crate::helpers::{Animation, Image, Scene};

#[derive(Debug, Serialize, Deserialize)]
pub struct PizzaStoneData {
    pub position: [f64; 2],
    pub visible: bool,
    pub scene: Scene,
}

pub struct PizzaStone {
    pub scene: Scene,
    image: Image,
    pub dx: f64,
    pub dy: f64,
    animation: Animation,
    pub used: bool,
    pub visible: bool,
    pub talk_used: Vec<Vec<[String; 2]>>
}

impl PizzaStone {
    pub fn new(data: PizzaStoneData) -> Self {
        Self {
            image: Image::new("images/characters/pizza-stone.png"),
            dx: data.position[0] * 16.0,
            dy: data.position[1] * 16.0,
            animation: Animation { count: 0, sx: 32.0, sy: 0.0 },
            used: false,
            scene: data.scene,
            visible: data.visible,
            talk_used: vec![vec![
                ["type".to_owned(), "textMessage".to_owned()],
                ["text".to_owned(), "Already used".to_owned()],
                ["repeat".to_owned(), "1".to_owned()],
            ]]
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, hero: [&f64; 2]) {
        if !self.visible {
            return;
        }
        
        if self.image.loaded() {
            ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                self.image.image_ref(),
                self.animation.sx,
                self.animation.sy,
                32f64,
                32f64,
                self.dx + 10.5 * 16f64 - hero[0] - 8f64,
                self.dy + 6f64 * 16f64 - hero[1] - 18f64,
                32f64,
                32f64,
            )
            .unwrap();
        }
    }

    pub fn update(&mut self) {
        // talk event sets it to true. after that this only needs to run once
        if self.used {
            self.animation.toggle_stone();
        }
    }
}
