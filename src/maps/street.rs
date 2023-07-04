use crate::helpers::{Direction, Event};

pub struct Street;

impl Street {
    pub const BACKGROUND_COLOR: &'static str = "black";
    pub const HERO_POSITION: [u16; 2] = [5, 11];
    pub const LOWER_IMAGE: &'static str = "images/maps/StreetLower.png";
    pub const UPPER_IMAGE: &'static str = "images/maps/StreetUpper.png";
    pub const NPCS: [(
        &'static str,
        &'static str,
        f64,
        f64,
        &'static [(Event, Direction, u16)],
    ); 2] = [
        (
            "Rachel",
            "images/characters/people/npc8.png",
            18.0 * 16.0,
            9.0 * 16.0,
            &[
                (Event::Stand, Direction::Down, 3),
                (Event::Walk, Direction::Down, 1),
                (Event::Walk, Direction::Right, 6),
                (Event::Walk, Direction::Down, 1),
                (Event::Stand, Direction::Down, 3),
                (Event::Walk, Direction::Up, 1),
                (Event::Walk, Direction::Left, 6),
                (Event::Walk, Direction::Up, 1),
            ],
        ),
        (
            "Ralph",
            "images/characters/people/npc7.png",
            33.0 * 16.0,
            13.0 * 16.0,
            &[
                (Event::Stand, Direction::Right, 3),
                (Event::Walk, Direction::Up, 3),
                (Event::Stand, Direction::Right, 3),
                (Event::Walk, Direction::Down, 3),
            ],
        ),
    ];
    pub const NPC_CUTSCENES: [(
        &'static str,
        &'static [(
            &'static [&'static str],
            &'static [&'static [[&'static str; 2]]],
        )],
    ); 2] = [
        (
            "Ralph",
            &[
                (
                    &[],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Ralph"],
                            ["text", "I wonder what's over there in the darkness. Can't seem to go in there."],
                            ["repeat", "1"],
                        ],
                    ]
                ),
            ],
        ),
        (
            "Rachel",
            &[
                (
                    &["DEFEATED_TINA"],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["speaker", "you"],
                            ["text", "I managed to defeat Tina."],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Rachel"],
                            ["text", "Good. You are worthy of my time then. Up for a battle?"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "replyMenu"],
                            ["enemy", "Rachel"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Tina"],
                            ["text", "Good luck! You're going to need it."],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "battle"],
                            ["enemyId", "Rachel"],
                            ["background", "images/maps/StreetBattle.png"],
                            ["repeat", "1"]
                        ],
                    ]
                ),
                (
                    &[],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["speaker", "you"],
                            ["text", "I want a battle."],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Rachel"],
                            ["text", "I don't battle with random strangers. Defeat Tina first to prove that you are worth my time."],
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
    ); 7] = [
        (
            [4, 10],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "DiningRoom"],
                    ["heroPosition", "6 11"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [4, 11],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "DiningRoom"],
                    ["heroPosition", "6 11"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [4, 12],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "DiningRoom"],
                    ["heroPosition", "6 11"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [4, 13],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "DiningRoom"],
                    ["heroPosition", "6 11"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [29, 9],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "PizzaShop"],
                    ["heroPosition", "5 11"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [5, 9],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "Kitchen"],
                    ["heroPosition", "5 9"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [25, 5],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "StreetNorth"],
                    ["heroPosition", "7 15"],
                    ["direction", "up"],
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
    pub const WALLS: [[u16; 2]; 88] = [
        [3, 10],
        [3, 11],
        [3, 12],
        [3, 13],
        [4, 14],
        [5, 14],
        [6, 14],
        [7, 14],
        [8, 14],
        [9, 14],
        [10, 14],
        [11, 14],
        [12, 14],
        [13, 14],
        [14, 14],
        [15, 14],
        [16, 14],
        [17, 14],
        [18, 14],
        [19, 14],
        [20, 14],
        [21, 14],
        [22, 14],
        [23, 14],
        [24, 14],
        [25, 14],
        [26, 14],
        [27, 14],
        [28, 14],
        [29, 14],
        [30, 14],
        [31, 14],
        [32, 14],
        [33, 14],
        [34, 13],
        [34, 12],
        [34, 11],
        [34, 10],
        [4, 9],
        [5, 8],
        [6, 9],
        [7, 9],
        [8, 9],
        [9, 9],
        [10, 9],
        [11, 9],
        [12, 9],
        [13, 8],
        [14, 8],
        [15, 7],
        [16, 7],
        [17, 7],
        [18, 7],
        [19, 7],
        [20, 7],
        [21, 7],
        [22, 7],
        [23, 7],
        [24, 7],
        [24, 6],
        [26, 6],
        [26, 5],
        [24, 5],
        [26, 7],
        [27, 7],
        [28, 8],
        [28, 9],
        [29, 8],
        [30, 9],
        [31, 9],
        [32, 9],
        [33, 9],
        // objects
        [16, 9],
        [16, 10],
        [16, 11],
        [17, 9],
        [17, 10],
        [17, 11],
        [18, 11],
        [19, 11],
        [25, 11],
        [25, 10],
        [25, 9],
        [26, 11],
        [26, 10],
        [26, 9],
        // npcs
        [18, 9],
        [33, 13],
    ];
}
