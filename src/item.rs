use web_sys::CanvasRenderingContext2d;

use crate::helpers::{Animation, Image};

pub struct Item {
    pub flag: String,
    image: Image,
    pub dx: f64,
    pub dy: f64,
    animation: Animation,
    pub visible: bool,
}

impl Item {
    pub fn new(item: &([u16; 2], &str), visible: bool) -> Self {
        Self {
            image: Image::new("images/characters/box.png"),
            dx: item.0[0] as f64 * 16.0,
            dy: item.0[1] as f64 * 16.0,
            animation: Animation::new(None),
            flag: item.1.to_string(),
            visible,
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
}
