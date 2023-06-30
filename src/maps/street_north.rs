use crate::helpers::{Direction, Event};

pub struct StreetNorth;

impl StreetNorth {
    pub const BACKGROUND_COLOR: &'static str = "black";
    pub const HERO_POSITION: [u16; 2] = [7, 15];
    pub const LOWER_IMAGE: &'static str = "images/maps/StreetNorthLower.png";
    pub const UPPER_IMAGE: &'static str = "images/maps/StreetNorthUpper.png";
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
            [7, 16],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "Street"],
                    ["heroPosition", "25 6"],
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
    pub const WALLS: [[u16; 2]; 48] = [
        [4, 5],
        [5, 5],
        [6, 5],
        [7, 4],
        [8, 5],
        [9, 5],
        [10, 5],
        [3, 6],
        [3, 7],
        [2, 7],
        [1, 8],
        [1, 9],
        [1, 10],
        [1, 11],
        [1, 12],
        [1, 13],
        [1, 14],
        [2, 15],
        [3, 15],
        [4, 15],
        [5, 15],
        [6, 15],
        [7, 18],
        [8, 15],
        [9, 15],
        [10, 15],
        [11, 15],
        [12, 15],
        [13, 15],
        [14, 14],
        [14, 13],
        [14, 12],
        [14, 11],
        [14, 10],
        [14, 9],
        [14, 8],
        [14, 7],
        [13, 6],
        [12, 6],
        [11, 6],
        // objects
        [7, 8],
        [7, 9],
        [7, 10],
        [8, 8],
        [8, 9],
        [8, 10],
        [9, 10],
        [10, 10],
    ];
}
