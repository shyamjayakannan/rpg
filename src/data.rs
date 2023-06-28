use crate::helpers::{Direction, Event};

pub const DATA: [(
    // lower image
    &str,
    // upper image
    &str,
    //walls
    &'static [[u8; 2]],
    // hero position
    [f64; 2],
    // npcs
    &'static [(&str, f64, f64, &[(Event, Direction, u8)])],
    // npc cutscenes
    &'static [&'static [(&'static [&str], &'static [&'static [[&str; 2]]])]],
    // special action locations
    &'static [[u8; 2]],
    // action location cutscenes
    &'static [&'static [(&'static [&str], &'static [&'static [[&str; 2]]])]],
    // pizza stones
    &'static [([u8; 2], &str, &'static [&'static [[&str; 2]]])],
    // items
    &'static [([u8; 2], &str, &'static [&'static [[&str; 2]]])],
); 2] = [
    (
        "./images/maps/DemoLower.png",
        "./images/maps/DemoUpper.png",
        &[
            [1, 3],
            [2, 3],
            [3, 3],
            [4, 3],
            [5, 3],
            [6, 4],
            [8, 4],
            [9, 3],
            [10, 3],
            [0, 4],
            [0, 5],
            [0, 6],
            [0, 7],
            [0, 8],
            [0, 9],
            [11, 4],
            [11, 5],
            [11, 6],
            [11, 7],
            [11, 8],
            [11, 9],
            [1, 10],
            [2, 10],
            [3, 10],
            [4, 10],
            [5, 11],
            [6, 10],
            [7, 10],
            [8, 10],
            [9, 10],
            [10, 10],
            [7, 6],
            [8, 6],
            [7, 7],
            [8, 7],
            [7, 3],
            // npc place
            [5, 7],
            // hero place
            [7, 4],
            // pizza stone place
            [2, 7],
        ],
        [7.0, 4.0],
        &[
            (
                "./images/characters/people/erio.png",
                5.0 * 16.0,
                7.0 * 16.0,
                &[(Event::Stand, Direction::Down, 1)],
            ),
        ],
        &[
            &[
                (
                    &["TALKED_TO_ERIO_1"],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "great! you can also skip to the end of the message by pressing ENTER if you're in a hurry."],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "in this world, you'll meet many people who will challenge you to pizza battles."],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "you will need pizzas to battle them. Let's make your first pizza. See that pizza stone over there?"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "cameraMove"],
                            ["location", "2 7"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "Walk up to it, then face it and press ENTER to choose a pizza. Then come back and talk to me"],
                            ["repeat", "1"],
                        ],
                    ]
                ),
                (
                    &["TALKED_TO_ERIO_2"],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "great! your battle lineup will appear at the top left of the screen."],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "you can create as many pizzas as you want but may use upto 3 during a battle"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "Also, you can use a pizza stone only once, so use it wisely."],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "Alright! Now, do you see that box over there?"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "cameraMove"],
                            ["location", "10 8"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "Walk up to it, then face it and press ENTER to open it."],
                            ["repeat", "1"],
                        ],
                    ]
                ),
                (
                    &["TALKED_TO_ERIO_3"],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "Let's have a practice battle"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "demoBattle"],
                            ["enemyId", "Erio"],
                            ["repeat", "1"]
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "Win battles to earn xp and level up your pizzas"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "You can also pause the game to save your data. press the ESC key to open or close the pause menu. Try saving your progress."],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "removeStoryFlag"],
                            ["flag", "TALKED_TO_ERIO_3"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "addStoryFlag"],
                            ["flag", "TALKED_TO_ERIO_4"],
                            ["repeat", "1"],
                        ],
                    ]
                ),
                (
                    &["TALKED_TO_ERIO_4"],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "Great! Pausing is also possible on your turn during a battle, to quit it. But remember that you will lose that battle's progress if you quit."],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["speaker", "Erio"],
                            ["text", "Alright! You've learnt enough to get you going in the world of PIZZA LEGENDS. Leave the demo room through the exit below to explore. Your adventure awaits!"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "addStoryFlag"],
                            ["flag", "TUTORIAL_COMPLETE"],
                            ["repeat", "1"],
                        ],
                    ]
                ),
            ]
        ],
        &[[7, 5]],
        &[
            &[
                (
                &["START"],
                &[
                    &[
                        ["type", "walk"],
                        ["direction", "left"],
                        ["who", "hero"],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "walk"],
                        ["direction", "down"],
                        ["who", "hero"],
                        ["repeat", "2"],
                    ],
                    &[
                        ["type", "stand"],
                        ["direction", "left"],
                        ["who", "hero"],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "stand"],
                        ["direction", "right"],
                        ["who", "0"],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "textMessage"],
                        ["speaker", "Erio"],
                        ["text", "Hi there! I am Erio. Welcome to the world of PIZZA LEGENDS! (press ENTER)"],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "textMessage"],
                        ["speaker", "Erio"],
                        ["text", "you can talk to people in PIZZA LEGENDS by walking next to them. Once you've reached, face the person and then press ENTER."],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "textMessage"],
                        ["speaker", "Erio"],
                        ["text", "try walking away and then come back to talk to me"],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "removeStoryFlag"],
                        ["flag", "START"],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "addStoryFlag"],
                        ["flag", "TALKED_TO_ERIO_1"],
                        ["repeat", "1"],
                    ],
                ],
            ),
            ],
        ],
        &[(
            [2, 7],
            "DEMO_ROOM_PIZZA_STONE",
            &[
                &[
                    ["type", "textMessage"],
                    ["text", "Use this Pizza Stone to make new pizzas"],
                    ["repeat", "1"],
                ],
                &[
                    ["index", "0"],
                    ["type", "craftingMenu"],
                    ["pizzas", "s001"],
                    ["repeat", "1"],
                ],
                &[
                    ["type", "removeStoryFlag"],
                    ["flag", "TALKED_TO_ERIO_1"],
                    ["repeat", "1"],
                ],
                &[
                    ["type", "addStoryFlag"],
                    ["flag", "TALKED_TO_ERIO_2"],
                    ["repeat", "1"],
                ],
                &[
                    ["type", "addStoryFlag"],
                    ["flag", "DEMO_ROOM_PIZZA_STONE"],
                    ["repeat", "1"],
                ],
            ]
        )],
        &[(
            [10, 8],
            "DEMO_ROOM_ITEM",
            &[
                &[
                    ["type", "textMessage"],
                    ["text", "You found Cheese!"],
                    ["repeat", "1"],
                ],
                &[
                    ["index", "0"],
                    ["type", "addItem"],
                    ["itemType", "item_recoverHp"],
                    ["repeat", "1"],
                ],
                &[
                    ["type", "removeStoryFlag"],
                    ["flag", "TALKED_TO_ERIO_2"],
                    ["repeat", "1"],
                ],
                &[
                    ["type", "addStoryFlag"],
                    ["flag", "TALKED_TO_ERIO_3"],
                    ["repeat", "1"],
                ],
                &[
                    ["type", "addStoryFlag"],
                    ["flag", "DEMO_ROOM_ITEM"],
                    ["repeat", "1"],
                ],
            ]
        )],
    ),
    (
        "./images/maps/KitchenLower.png",
        "./images/maps/KitchenUpper.png",
        &[
            [1, 3],
            [2, 3],
            [3, 3],
            [4, 3],
            [5, 3],
            [6, 4],
            [8, 4],
            [9, 3],
            [10, 3],
            [0, 4],
            [0, 5],
            [0, 6],
            [0, 7],
            [0, 8],
            [0, 9],
            [11, 4],
            [11, 5],
            [11, 6],
            [11, 7],
            [11, 8],
            [11, 9],
            [1, 10],
            [2, 10],
            [3, 10],
            [4, 10],
            [5, 11],
            [6, 10],
            [7, 10],
            [8, 10],
            [9, 10],
            [10, 10],
            [7, 6],
            [8, 6],
            [7, 7],
            [8, 7],
            [7, 3],
            // npc place
            [8, 5],
            [5, 8],
            //hero place
            [5, 5],
        ],
        [5.0, 5.0],
        &[(
            "./images/characters/people/erio.png",
            8.0 * 16.0,
            5.0 * 16.0,
            &[(Event::Stand, Direction::Down, 1)],
        )],
        &[
            &[(
                &[],
                &[
                    &[
                        ["type", "textMessage"],
                        ["text", "Hello there"],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "textMessage"],
                        ["text", "go away"],
                        ["repeat", "1"],
                    ],
                ],
            )],
            &[
                (
                    &["TALKED_TO_ERIO"],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["text", "Hello there"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["text", "let's battle"],
                            ["repeat", "1"],
                        ],
                        &[["type", "battle"], ["enemyId", "beth"], ["repeat", "1"]],
                    ],
                ),
                (
                    &[],
                    &[
                        &[
                            ["type", "textMessage"],
                            ["text", "Hello there"],
                            ["repeat", "1"],
                        ],
                        &[
                            ["type", "textMessage"],
                            ["text", "go away"],
                            ["repeat", "1"],
                        ],
                    ],
                ),
            ],
        ],
        &[[7, 4], [5, 10]],
        &[
            &[(
                &[],
                &[
                    &[
                        ["type", "walk"],
                        ["direction", "left"],
                        ["who", "0"],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "stand"],
                        ["direction", "up"],
                        ["who", "0"],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "textMessage"],
                        ["text", "go away"],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "walk"],
                        ["direction", "right"],
                        ["who", "0"],
                        ["repeat", "1"],
                    ],
                    &[
                        ["type", "stand"],
                        ["direction", "down"],
                        ["who", "0"],
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
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "0"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ],
        &[],
        &[],
    ),
];
