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
    ); 1] = [(
        "Tina",
        "images/characters/people/npc4.png",
        11.0 * 16.0,
        8.0 * 16.0,
        &[
            (Event::Stand, Direction::Up, 3),
            (Event::Walk, Direction::Right, 2),
            (Event::Stand, Direction::Right, 3),
            (Event::Walk, Direction::Down, 2),
            (Event::Stand, Direction::Down, 3),
            (Event::Walk, Direction::Left, 2),
            (Event::Stand, Direction::Left, 3),
            (Event::Walk, Direction::Up, 2),
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
            "Tina",
            &[
                (
                    &[],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Tina"],
                            ["text", "Would you like to battle?"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "replyMenu"],
                            ["enemy", "Tina"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Tina"],
                            ["text", "Great! Let's battle."],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "battle"],
                            ["enemyId", "Tina"],
                            ["background", "images/maps/PizzaShopBattle.png"],
                            ["repeat", "1"]
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Tina"],
                            ["text", "Challenge Rachel if you're feeling confident. She is a veggie pizza expert."],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "addStoryFlag"],
                            ["flag", "DEFEATED_TINA"],
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
        (
            [7, 5],
            &[
                (
                    &["LEVEL_2"],
                    &[&[
                        ["type", "changeMap"],
                        ["map", "GreenKitchen"],
                        ["heroPosition", "5 10"],
                        ["direction", "up"],
                        ["repeat", "1"],
                    ]],
                ),
                (
                    &[],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Unknown"],
                            ["text", "No entry for level 1 chumps."],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Unknown"],
                            ["text", "Now clear out!"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "walk"],
                            ["direction", "down"],
                            ["who", "hero"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "walk"],
                            ["direction", "left"],
                            ["who", "hero"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "stand"],
                            ["direction", "down"],
                            ["who", "hero"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "you"],
                            ["text", "Whew! How rude!"],
                            ["repeat", "1"],
                        ],
                    ],
                ),
            ],
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
    pub const WALLS: [[u16; 2]; 49] = [
        [4, 5],
        [5, 5],
        [6, 5],
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
        // npcs
        [7, 4],
        [11, 8],
    ];
}
