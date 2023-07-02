export const Pizzatypes = {
    normal: "normal",
    spicy: "spicy",
    veggie: "veggie",
    fungi: "fungi",
    chill: "chill",
}

export const Pizzas = {
    "s001": {
        name: "Slice Samurai",
        type: Pizzatypes.spicy,
        description: "simple starter pizza",
        src: "images/characters/pizzas/s001.png",
        icon: "images/icons/spicy.png",
        actions: ["damage1"],
    },
    "s002": {
        name: "Bacon Brigade",
        description: "woo",
        type: Pizzatypes.spicy,
        src: "images/characters/pizzas/s002.png",
        icon: "images/icons/spicy.png",
        actions: ["damage1", "saucyStatus"],
    },
    "s003": {
        name: "Chilli Extreme",
        description: "woo",
        type: Pizzatypes.spicy,
        src: "images/characters/pizzas/s002.png",
        icon: "images/icons/spicy.png",
        actions: ["damage1", "saucyStatus", "clumsyStatus"],
    },
    "v001": {
        name: "Call me Kale",
        type: Pizzatypes.veggie,
        src: "images/characters/pizzas/v001.png",
        icon: "images/icons/veggie.png",
        actions: ["damage1"],
    },
    "f001": {
        name: "Portobello Express",
        type: Pizzatypes.spicy,
        src: "images/characters/pizzas/f001.png",
        icon: "images/icons/fungi.png",
        actions: ["damage1"],
    },
}