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
                status: { type: "saucy", expiresIn: 3 },
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
    "Sam": {
        name: "Sam",
        src: "images/characters/people/npc2.png",
        pizzas: {
            "a": {
                pizzaId: "s001",
                maxHp: 50,
                level: 1,
            },
        },
        message: {
            won: [
                { type: "textMessage", text: "Ha ha! You lost.", speaker: "Sam", repeat: 1 },
            ],
            lost: [
                { type: "textMessage", text: "Impossible! I can't lose. I am going to sulk.", speaker: "Sam", repeat: 1 },
            ],
            quit: [
                { type: "textMessage", text: "Hey! I was winning that.", speaker: "Sam", repeat: 1 },
            ],
            battleDeclined: [
                { type: "textMessage", text: "Yes! Fear me.", speaker: "Sam", repeat: 1 },
            ],
        },
    },
    "Tina": {
        name: "Tina",
        src: "images/characters/people/npc4.png",
        pizzas: {
            "a": {
                pizzaId: "s003",
                maxHp: 50,
                level: 3,
            },
        },
        message: {
            won: [
                { type: "textMessage", text: "Come back when you're stronger!", speaker: "Tina", repeat: 1 },
            ],
            lost: [
                { type: "textMessage", text: "You're pretty good at this.", speaker: "Tina", repeat: 1 },
            ],
            quit: [
                { type: "textMessage", text: "Quitting during battles should be banned.", speaker: "Tina", repeat: 1 },
            ],
            battleDeclined: [
                { type: "textMessage", text: "Alright! See you.", speaker: "Tina", repeat: 1 },
            ],
        },
    },
    "Rachel": {
        name: "Rachel",
        src: "images/characters/people/npc8.png",
        pizzas: {
            "a": {
                pizzaId: "v002",
                maxHp: 50,
                level: 2,
            },
        },
        message: {
            won: [
                { type: "textMessage", text: "What a waste of time. That was so easy!", speaker: "Rachel", repeat: 1 },
            ],
            lost: [
                { type: "textMessage", text: "Wow! Not many can beat me, you know.", speaker: "Rachel", repeat: 1 },
            ],
            quit: [
                { type: "textMessage", text: "Show some sportsmanship. If you're losing, lose with dignity.", speaker: "Rachel", repeat: 1 },
            ],
            battleDeclined: [
                { type: "textMessage", text: "Hmm, this place is full of weaklings!", speaker: "Rachel", repeat: 1 },
            ],
        },
    },
};