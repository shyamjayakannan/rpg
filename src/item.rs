use web_sys::CanvasRenderingContext2d;

use crate::{
    data::DATA,
    helpers::{Animation, Image},
};

pub struct Item {
    pub flag: String,
    image: Image,
    pub dx: f64,
    pub dy: f64,
    animation: Animation,
    pub item_event: &'static [&'static [[&'static str; 2]]],
}

impl Item {
    pub fn new(index: usize, id: usize) -> Self {
        Self {
            image: Image::new("./images/characters/box.png"),
            dx: DATA[index].9[id].0[0] as f64 * 16.0,
            dy: DATA[index].9[id].0[1] as f64 * 16.0,
            animation: Animation::new(),
            item_event: DATA[index].9[id].2,
            flag: DATA[index].8[id].1.to_string(),
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
}
