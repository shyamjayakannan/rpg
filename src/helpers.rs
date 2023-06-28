use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlImageElement;

pub struct Image {
    image: HtmlImageElement,
}

impl Image {
    pub fn new(src: &str) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(src);

        Self { image }
    }

    pub fn loaded(&self) -> bool {
        self.image.complete()
    }

    pub fn image_ref(&self) -> &HtmlImageElement {
        &self.image
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
pub enum Event {
    Walk,
    Stand,
}

pub struct DirectionInput {
    pub held_directions: Vec<Direction>,
}

impl DirectionInput {
    pub const MAP: [Direction; 4] = [
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::Up,
    ];

    pub fn new() -> Self {
        Self {
            held_directions: Vec::with_capacity(4),
        }
    }

    pub fn add(&mut self, direction_number: usize) {
        if !self.held_directions.contains(&Self::MAP[direction_number]) {
            self.held_directions.push(Self::MAP[direction_number].clone());
        }
    }

    pub fn remove(&mut self, direction_number: usize) {
        let index = self
            .held_directions
            .iter()
            .position(|e| *e == Self::MAP[direction_number])
            .unwrap();
        self.held_directions.remove(index);
    }
}


pub fn get_direction(direction: &str) -> Direction {
    match direction {
        "left" => Direction::Left,
        "right" => Direction::Right,
        "up" => Direction::Up,
        _ => Direction::Down,
    }
}

pub struct Movement {
    pub progress_remaining: u8,
    pub moveable: bool,
}

impl Movement {
    pub fn new(starting_progress: u8) -> Self {
        Self {
            progress_remaining: starting_progress,
            moveable: true,
        }
    }

    pub fn can_move(&mut self, walls: &mut Vec<[u8; 2]>, dx: &f64, dy: &f64, direction: &Direction) {
        let elem = get_next_place(direction, dx, dy);

        self.moveable = match walls.contains(&elem) {
            true => false,
            false => {
                walls.remove(
                    walls
                        .iter()
                        .position(|pair| *pair == [*dx as u8, *dy as u8])
                        .unwrap_throw(),
                );
                walls.push(elem);
                true
            }
        }
    }
}

pub fn get_next_place(direction: &Direction, dx: &f64, dy: &f64) -> [u8; 2] {
    match direction {
        Direction::Down => [*dx as u8, *dy as u8 + 16],
        Direction::Up => [*dx as u8, *dy as u8 - 16],
        Direction::Right => [*dx as u8 + 16, *dy as u8],
        Direction::Left => [*dx as u8 - 16, *dy as u8],
    }
}

pub fn get_opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Down => Direction::Up,
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Left => Direction::Right,
    }
}

pub struct Animation {
    pub count: usize,
    pub sx: f64,
    pub sy: f64,
}

impl Animation {
    pub const FRAMES_PER_STEP: u8 = 8;

    pub const STEPS: [[[u8; 2]; 4]; 4] = [
        [[0, 0], [32, 0], [0, 0], [96, 0]],
        [[0, 32], [32, 32], [0, 32], [96, 32]],
        [[0, 64], [32, 64], [0, 64], [96, 64]],
        [[0, 96], [32, 96], [0, 96], [96, 96]],
    ];

    pub const STONE_STEPS: [[u8; 2]; 2] = [[32, 0], [0, 0]];

    pub fn new() -> Self {
        Self {
            count: 0,
            sx: 0f64,
            sy: 0f64,
        }
    }

    pub fn selected_frame(&mut self, direction: &Direction, count: usize) {
        let index = match direction {
            Direction::Down => 0,
            Direction::Right => 1,
            Direction::Up => 2,
            Direction::Left => 3,
        };

        self.sx = Self::STEPS[index][count][0] as f64;
        self.sy = Self::STEPS[index][count][1] as f64;
    }

    pub fn toggle(&mut self, direction: &Direction) {
        let index = match direction {
            Direction::Down => 0,
            Direction::Right => 1,
            Direction::Up => 2,
            Direction::Left => 3,
        };

        match &mut self.count {
            x if *x < Self::STEPS.len() - 1 => *x += 1,
            x => *x = 0,
        }

        self.sx = Self::STEPS[index][self.count][0] as f64;
        self.sy = Self::STEPS[index][self.count][1] as f64;
    }
    
    pub fn toggle_stone(&mut self) {
        self.sx = Self::STONE_STEPS[1][0] as f64;
        self.sy = Self::STONE_STEPS[1][1] as f64;
    }
}
