use crate::helpers::{Direction, Event};

pub struct GreenKitchen;

impl GreenKitchen {
    pub const BACKGROUND_COLOR: &'static str = "black";
    pub const HERO_POSITION: [u16; 2] = [5, 11];
    pub const LOWER_IMAGE: &'static str = "images/maps/GreenKitchenLower.png";
    pub const UPPER_IMAGE: &'static str = "images/maps/GreenKitchenUpper.png";
    pub const NPCS: [(
        &'static str,
        &'static str,
        f64,
        f64,
        &'static [(Event, Direction, u16)],
    ); 0] = [];
    pub const NPC_CUTSCENES: [(
        &'static str,
        &'static [(
            &'static [&'static str],
            &'static [&'static [[&'static str; 2]]],
        )],
    ); 0] = [];
    pub const ACTION_CUTSCENES: [(
        [u16; 2],
        &'static [(
            &'static [&'static str],
            &'static [&'static [[&'static str; 2]]],
        )],
    ); 1] = [
        (
            [5, 12],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "StreetNorth"],
                    ["heroPosition", "7 6"],
                    ["direction", "down"],
                    ["repeat", "1"],
                ]],
            )],
        ),
    ];
    pub const PIZZA_STONES: [(
        [u16; 2],
        &'static str,
        &'static [&'static [[&'static str; 2]]],
    ); 0] = [];
    pub const ITEMS: [(
        [u16; 2],
        &'static str,
        &'static [&'static [[&'static str; 2]]],
    ); 0] = [];
    pub const WALLS: [[u16; 2]; 49] = [
        [0, 4],
        [0, 5],
        [0, 6],
        [0, 7],
        [0, 8],
        [0, 9],
        [0, 10],
        [0, 11],
        [1, 12],
        [2, 12],
        [3, 12],
        [4, 12],
        [5, 13],
        [6, 12],
        [6, 12],
        [8, 12],
        [9, 12],
        [10, 5],
        [10, 6],
        [10, 7],
        [10, 8],
        [10, 9],
        [10, 10],
        [10, 11],
        [9, 4],
        [8, 4],
        [1, 3],
        [2, 3],
        [3, 3],
        [4, 3],
        [5, 3],
        [6, 3],
        [7, 3],
        // objects
        [1, 6],
        [2, 6],
        [3, 6],
        [4, 6],
        [5, 6],
        [6, 6],
        [8, 5],
        [3, 7],
        [4, 7],
        [6, 7],
        [2, 9],
        [3, 9],
        [4, 9],
        [7, 10],
        [8, 10],
        [9, 10],
    ];
}
