use web_sys::CanvasRenderingContext2d;

use crate::{
    data::DATA,
    helpers::{Animation, Image},
};

pub struct PizzaStone {
    pub flag: String,
    image: Image,
    pub dx: f64,
    pub dy: f64,
    animation: Animation,
    pub used: bool,
}

impl PizzaStone {
    pub const TALK_USED: [&'static [[&'static str; 2]]; 1] = [&[
        ["type", "textMessage"],
        ["text", "Already used"],
        ["repeat", "1"],
    ]];

    pub fn new(pizza_stone: &([u8; 2], &str)) -> Self {
        Self {
            image: Image::new("images/characters/pizza-stone.png"),
            dx: pizza_stone.0[0] as f64 * 16.0,
            dy: pizza_stone.0[1] as f64 * 16.0,
            animation: Animation { count: 0, sx: 32.0, sy: 0.0 },
            used: false,
            flag: pizza_stone.1.to_string(),
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, hero: [&f64; 2]) {
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
