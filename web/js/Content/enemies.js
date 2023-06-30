export const Enemies = {
    "Erio": {
        name: "Erio",
        src: "images/characters/people/erio.png",
        pizzas: {
            "a": {
                pizzaId: "s001",
                maxHp: 50,
                level: 1,
            },
            "b": {
                pizzaId: "s002",
                maxHp: 50,
                level: 1,
                status: { type: "saucy" },
            },
        },
        message: {
            won: [
                { type: "textMessage", text: "you loser!", speaker: "Erio", repeat: 1 },
            ],
            lost: [
                { type: "textMessage", text: "you are good", speaker: "Erio", repeat: 1 },
            ],
            quit: [
                { type: "textMessage", text: "i hate quitters!", speaker: "Erio", repeat: 1 },
            ],
        },
    },
    "Beth": {
        name: "Beth",
        src: "images/characters/people/npc1.png",
        pizzas: {
            "a": {
                hp: 1,
                pizzaId: "s001",
                maxHp: 50,
                level: 1,
            },
        },
        message: {
            won: [
                { type: "textMessage", text: "you loser!", speaker: "Beth", repeat: 1 },
            ],
            lost: [
                { type: "textMessage", text: "you are good", speaker: "Beth", repeat: 1 },
            ],
            quit: [
                { type: "textMessage", text: "i hate quitters!", speaker: "Beth", repeat: 1 },
            ],
        },
    },
};