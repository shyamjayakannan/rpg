use crate::helpers::{Direction, Event};

pub struct Kitchen;

impl Kitchen {
    pub const BACKGROUND_COLOR: &'static str = "black";
    pub const HERO_POSITION: [u16; 2] = [5, 9];
    pub const LOWER_IMAGE: &'static str = "images/maps/KitchenLower.png";
    pub const UPPER_IMAGE: &'static str = "images/maps/KitchenUpper.png";
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
            [5, 10],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "Street"],
                    ["heroPosition", "5 10"],
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
    pub const WALLS: [[u16; 2]; 36] = [
        [1, 5],
        [1, 6],
        [1, 7],
        [0, 8],
        [1, 9],
        [2, 9],
        [3, 10],
        [4, 10],
        [5, 11],
        [6, 10],
        [7, 10],
        [8, 10],
        [9, 9],
        [10, 9],
        [11, 10],
        [12, 10],
        [13, 9],
        [13, 8],
        [13, 7],
        [13, 6],
        [13, 5],
        [12, 4],
        [11, 4],
        [10, 3],
        [9, 3],
        [8, 3],
        [7, 3],
        [6, 3],
        [5, 3],
        [4, 3],
        [3, 3],
        [2, 3],
        // objects
        [6, 7],
        [7, 7],
        [9, 7],
        [10, 7],
    ];
}
