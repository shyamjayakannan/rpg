use std::mem::swap;

use crate::emit_event;

pub struct Camera {
    pub dx: f64,
    pub dy: f64,
    pub inc_dx: f64,
    pub inc_dy: f64,
    wait: u8,
    fixed_initial_location: [f64; 2],
    initial_location: Option<[f64; 2]>,
    final_location: Option<[f64; 2]>,
}

impl Camera {
    // factor by which camera is slower than other characters
    pub const SPEED: f64 = 16.0;
    pub const WAIT: u8 = 64;

    pub fn new(dx: f64, dy: f64) -> Self {
        Self {
            dx,
            dy,
            inc_dx: 0.0,
            inc_dy: 0.0,
            initial_location: None,
            fixed_initial_location: [dx, dy],
            final_location: None,
            wait: 0,
        }
    }

    pub fn update(&mut self, hero: [&f64; 2]) {
        self.dx = *hero[0];
        self.dy = *hero[1];
    }

    pub fn set_motion(&mut self, final_location: [f64; 2]) {
        self.fixed_initial_location = [self.dx, self.dy];
        self.initial_location = Some([self.dx, self.dy]);
        self.final_location = Some([final_location[0] * 16.0, final_location[1] * 16.0]);

        self.inc_dx = (final_location[0] * 16.0 - self.dx) / Self::SPEED;
        self.inc_dy = (final_location[1] * 16.0 - self.dy) / Self::SPEED;
    }

    pub fn move_camera(&mut self) {
        if let Some(x) = self.final_location {
            if self.dy != x[1] || self.dx != x[0] {
                self.dx += self.inc_dx;
                self.dy += self.inc_dy;

                return;
            } else {
                if self.fixed_initial_location[0] == self.dx && self.fixed_initial_location[1] == self.dy {
                    self.final_location = None;
                    self.initial_location = None;
                    self.wait = 0;

                    emit_event("Complete", "camera");

                    return;
                } else if self.wait == Self::WAIT {
                    swap(&mut self.initial_location, &mut self.final_location);

                    // come back twice as fast
                    self.inc_dx *= -2.0;
                    self.inc_dy *= -2.0;
                    self.wait = 0;

                    return;
                }

                self.wait += 1;
            }
        }
    }
}
