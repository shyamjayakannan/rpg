use crate::helpers::{Direction, Event};

pub struct PizzaShop;

impl PizzaShop {
    pub const BACKGROUND_COLOR: &'static str = "black";
    pub const HERO_POSITION: [u16; 2] = [5, 11];
    pub const LOWER_IMAGE: &'static str = "images/maps/PizzaShopLower.png";
    pub const UPPER_IMAGE: &'static str = "images/maps/PizzaShopUpper.png";
    pub const NPCS: [(
        &'static str,
        &'static str,
        f64,
        f64,
        &'static [(Event, Direction, u16)],
    ); 1] = [(
        "Pamela",
        "images/characters/people/npc1.png",
        10.0 * 16.0,
        4.0 * 16.0,
        &[
            (Event::Stand, Direction::Left, 3),
            (Event::Walk, Direction::Down, 3),
            (Event::Walk, Direction::Left, 1),
            (Event::Walk, Direction::Down, 2),
            (Event::Stand, Direction::Left, 3),
            (Event::Walk, Direction::Down, 1),
            (Event::Walk, Direction::Left, 4),
            (Event::Stand, Direction::Left, 3),
            (Event::Walk, Direction::Up, 3),
            (Event::Walk, Direction::Right, 5),
            (Event::Walk, Direction::Up, 3),
        ],
    )];
    pub const NPC_CUTSCENES: [(
        &'static str,
        &'static [(
            &'static [&'static str],
            &'static [&'static [[&'static str; 2]]],
        )],
    ); 1] = [
        (
            "Pamela",
            &[
                (
                    &[],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Pamela"],
                            ["text", "You can buy pizza accessories here."],
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
    ); 2] = [
        (
            [5, 12],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "Street"],
                    ["heroPosition", "29 10"],
                    ["direction", "down"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [6, 6],
            &[(
                &[],
                &[
                    &[
                        ["type", "textMessage"],
                        ["speaker", "Pamela"],
                        ["text", "Oi! You aren't allowed in there."],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "walk"],
                        ["direction", "down"],
                        ["who", "hero"],
                        ["repeat", "1"],
                    ],
                ],
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
    pub const WALLS: [[u16; 2]; 56] = [
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
        [7, 12],
        [8, 12],
        [9, 12],
        [10, 12],
        [11, 11],
        [11, 10],
        [11, 9],
        [11, 8],
        [11, 7],
        [11, 6],
        [11, 5],
        [11, 4],
        [1, 3],
        [3, 3],
        [4, 3],
        [5, 3],
        [6, 3],
        [7, 3],
        [8, 3],
        [10, 3],
        // objects
        [2, 4],
        [2, 5],
        [2, 6],
        [3, 6],
        [4, 6],
        [5, 6],
        [7, 6],
        [8, 6],
        [9, 6],
        [9, 4],
        [9, 5],
        [7, 8],
        [7, 9],
        [8, 9],
        [8, 8],
        [3, 8],
        [3, 9],
        [3, 10],
        [4, 8],
        [4, 9],
        [4, 10],
        // npc
        [10, 4],
    ];
}
