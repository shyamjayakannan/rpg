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
    ); 1] = [
        (
            "Tony",
            "images/characters/people/npc5.png",
            6.0 * 16.0,
            6.0 * 16.0,
            &[
                (Event::Stand, Direction::Down, 3),
                (Event::Walk, Direction::Up, 1),
                (Event::Stand, Direction::Up, 3),
                (Event::Walk, Direction::Right, 3),
                (Event::Walk, Direction::Up, 1),
                (Event::Stand, Direction::Up, 3),
                (Event::Walk, Direction::Right, 1),
                (Event::Walk, Direction::Down, 2),
                (Event::Stand, Direction::Down, 3),
                (Event::Walk, Direction::Right, 1),
                (Event::Walk, Direction::Down, 3),
                (Event::Stand, Direction::Left, 3),
                (Event::Walk, Direction::Up, 1),
                (Event::Walk, Direction::Left, 6),
                (Event::Walk, Direction::Up, 2),
                (Event::Walk, Direction::Right, 1),
            ],
        ),
    ];
    pub const NPC_CUTSCENES: [(
        &'static str,
        &'static [(
            &'static [&'static str],
            &'static [&'static [[&'static str; 2]]],
        )],
    ); 1] = [
        (
            "Tony",
            &[
                (
                    &[],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Tony"],
                            ["text", "Welcome to Tony Boiii's."],
                            ["repeat", "1"],
                        ],
                    ]
                ),
            ],
        ),
    ];
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
        bool,
        [u16; 2],
        &'static str,
        &'static [&'static [[&'static str; 2]]],
    ); 0] = [];
    pub const ITEMS: [(
        bool,
        [u16; 2],
        &'static str,
        &'static [&'static [[&'static str; 2]]],
    ); 0] = [];
    pub const WALLS: [[u16; 2]; 38] = [
        [1, 4],
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
        // npcs
        [6, 6],
    ];
}
