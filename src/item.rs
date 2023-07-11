use serde::{Serialize, Deserialize};
use web_sys::CanvasRenderingContext2d;

use crate::helpers::{Animation, Image, Scene};

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemData {
    pub position: [f64; 2],
    pub visible: bool,
    pub scene: Scene,
}

pub struct Item {
    pub scene: Scene,
    image: Image,
    pub dx: f64,
    pub dy: f64,
    animation: Animation,
    pub visible: bool,
}

impl Item {
    pub fn new(data: ItemData) -> Self {
        Self {
            image: Image::new("images/characters/box.png"),
            dx: data.position[0] * 16.0,
            dy: data.position[1] * 16.0,
            animation: Animation::new(None),
            scene: data.scene,
            visible: data.visible,
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
