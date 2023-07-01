use crate::{
    helpers::{Direction, Event, Image, Movement, Animation},
    emit_event
};
use web_sys::CanvasRenderingContext2d;

pub struct Hero {
    image: Image,
    shadow: Image,
    pub dx: f64,
    pub dy: f64,
    pub movement: Movement,
    pub direction: Direction,
    animation: Animation,
    pub cutscene: Option<(Event, Direction)>,
}

impl Hero {
    pub fn new(src: &str, x: f64, y: f64, direction: Direction) -> Self {
        Self {
            image: Image::new(src),
            shadow: Image::new("images/characters/shadow.png"),
            dx: x,
            dy: y,
            movement: Movement::new(0),
            animation: Animation::new(Some(&direction)),
            direction,
            cutscene: None,
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, hero: [&f64; 2]) {
        if self.shadow.loaded() {
            ctx.draw_image_with_html_image_element(
                self.shadow.image_ref(),
                self.dx + 10.5 * 16f64 - hero[0] - 8f64,
                self.dy + 6f64 * 16f64 - hero[1] - 18f64,
            )
            .unwrap();

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

    pub fn update(&mut self, direction_vec: &Vec<Direction>, walls: &mut Vec<[u16; 2]>) {
        if self.movement.progress_remaining > 0 {
            if self.movement.moveable {
                match self.direction {
                    Direction::Down => self.dy += 1.0,
                    Direction::Up => self.dy -= 1.0,
                    Direction::Left => self.dx -= 1.0,
                    Direction::Right => self.dx += 1.0,
                }
            };

            if self.movement.progress_remaining % Animation::FRAMES_PER_STEP == 0 {
                self.animation.toggle(&self.direction);
            }

            self.movement.progress_remaining -= 1;

            if self.movement.progress_remaining == 0 {
                emit_event("HeroWalkingComplete", "hero");
            }

            return;
        } else {

            let vec = direction_vec;

            if vec.len() > 0 {
                self.direction = vec[vec.len() - 1].clone();

                self.movement
                    .can_move(walls, &self.dx, &self.dy, &self.direction);
                self.movement.progress_remaining = 16;
            }
        }
    }

    pub fn update_cutscene(&mut self, walls: &mut Vec<[u16; 2]>) {
        let x = match &self.cutscene {
            Some(x) => x,
            None => return,
        };

        match x.0 {
            Event::Walk => {
                if self.movement.progress_remaining == 16 {
                    self.movement.can_move(walls, &self.dx, &self.dy, &x.1);
                }

                if self.movement.moveable {
                    match x.1 {
                        Direction::Down => self.dy += 1.0,
                        Direction::Up => self.dy -= 1.0,
                        Direction::Left => self.dx -= 1.0,
                        Direction::Right => self.dx += 1.0,
                    }
                };
                
                if self.movement.progress_remaining % Animation::FRAMES_PER_STEP == 0 {
                    self.animation.toggle(&x.1);
                }
            }
            Event::Stand => {
                if self.movement.progress_remaining == 16 {
                    self.animation.selected_frame(&x.1, 0);
                }
            }
        }

        self.movement.progress_remaining -= 1;

        if self.movement.progress_remaining == 0 {
            emit_event("Complete", "hero");
        }
    }
}
