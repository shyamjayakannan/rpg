use serde::{Deserialize, Serialize};
use web_sys::CanvasRenderingContext2d;

use crate::{
    emit_event,
    helpers::{Action, Animation, Direction, Event, Image, Movement},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct NpcData {
    pub name: String,
    pub image: String,
    pub dx: f64,
    pub dy: f64,
    pub actions: Vec<Action>,
}

pub struct Npc {
    pub name: String,
    image: Image,
    shadow: Image,
    pub dx: f64,
    pub dy: f64,
    pub movement: Movement,
    animation: Animation,
    actions: Vec<Action>,
    action_index: usize,
    pub direction: Direction,
    repeat_count: u16,
    current_position: [f64; 2],
    pub cutscene: Option<(Event, Direction)>,
}

impl Npc {
    pub fn new(data: NpcData) -> Self {
        Self {
            name: data.name,
            image: Image::new(&data.image),
            shadow: Image::new("images/characters/shadow.png"),
            dx: data.dx,
            dy: data.dy,
            movement: Movement::new(0),
            animation: Animation::new(Some(&data.actions[0].direction)),
            direction: data.actions[0].direction.clone(),
            action_index: 0,
            actions: data.actions,
            current_position: [data.dx, data.dy],
            repeat_count: 0,
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

    pub fn update(&mut self, walls: &mut Vec<[u16; 2]>) {
        let Action {
            ref event,
            ref direction,
            ref repeat,
        } = self.actions[self.action_index];

        if self.movement.progress_remaining == 0 {
            self.movement.progress_remaining = 16;

            if *event == Event::Stand
                || (*event == Event::Walk && self.current_position != [self.dx, self.dy])
            {
                self.repeat_count += 1;
                self.current_position = [self.dx, self.dy];

                if *repeat == self.repeat_count {
                    self.repeat_count = 0;
                    self.action_index += 1;
                }
            }

            if self.action_index == self.actions.len() {
                self.action_index = 0;
            }

            return;
        }

        match event {
            Event::Walk => {
                if self.movement.progress_remaining == 16 {
                    self.direction = direction.clone();
                    self.movement
                        .can_move(walls, &self.dx, &self.dy, &self.direction);
                }

                if self.movement.moveable {
                    match &self.direction {
                        Direction::Down => self.dy += 1.0,
                        Direction::Up => self.dy -= 1.0,
                        Direction::Left => self.dx -= 1.0,
                        Direction::Right => self.dx += 1.0,
                    }
                };

                if self.movement.progress_remaining % Animation::FRAMES_PER_STEP == 0 {
                    self.animation.toggle(&self.direction);
                }
            }
            Event::Stand => {
                if self.movement.progress_remaining == 16 {
                    self.direction = direction.clone();
                    self.animation.selected_frame(&self.direction, 0);
                }
            }
        }

        self.movement.progress_remaining -= 1;
    }

    pub fn update_cutscene(&mut self, walls: &mut Vec<[u16; 2]>) {
        let x = match &self.cutscene {
            Some(x) => x,
            None => {
                self.animation.selected_frame(&self.direction, 0);
                return;
            }
        };

        match &x.0 {
            Event::Walk => {
                if self.movement.progress_remaining == 16 {
                    self.movement.can_move(walls, &self.dx, &self.dy, &x.1);
                }

                if self.movement.moveable {
                    match &x.1 {
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
            self.movement.progress_remaining = 16;
            emit_event("Complete", &self.name);
        }
    }
}
