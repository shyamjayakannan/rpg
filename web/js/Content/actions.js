export const Actions = {
    damage1: {
        name: "whomp!",
        description: "Decreases target's hp by 10",
        success: [
            { type: "textMessage", text: "{CASTER} uses {ACTION}!" },
            { type: "animation", animation: "spin" },
            { type: "stateChange", damage: 10 },
        ]
    },
    damage2: {
        name: "power whomp!",
        description: "Decreases target's hp by 15",
        success: [
            { type: "textMessage", text: "{CASTER} uses {ACTION}!" },
            { type: "animation", animation: "spin" },
            { type: "stateChange", damage: 15 },
        ]
    },
    saucyStatus: {
        name: "Tomato Squeeze!",
        targetType: "friendly",
        description: "recovers 3 hp after every turn for 3 turns",
        success: [
            { type: "textMessage", text: "{CASTER} uses {ACTION}!" },
            { type: "stateChange", status: { type: "saucy", expiresIn: 3 } },
        ]
    },
    reboundStatus: {
        name: "Paprika shield!",
        targetType: "friendly",
        description: "Rebounds enemy's attcks for 2 turns",
        success: [
            { type: "textMessage", text: "{CASTER} uses {ACTION}!" },
            { type: "stateChange", status: { type: "rebounding", expiresIn: 2 } },
        ]
    },
    clumsyStatus: {
        name: "Olive Oil!",
        description: "Increases target's probability of missing attacks",
        success: [
            { type: "textMessage", text: "{CASTER} uses {ACTION}!" },
            { type: "animation", animation: "glob", color: "#dafd2a" },
            { type: "stateChange", status: { type: "clumsy", expiresIn: 3 } },
            { type: "textMessage", text: "{TARGET} is slipping!" },
        ]
    },
    removeStatus: {
        name: "Pepper Spray!",
        description: "removes user's pizza status",
        success: [
            { type: "textMessage", text: "{CASTER} uses {ACTION}!" },
            { type: "animation", animation: "glob", color: "#325aa8" },
            { type: "stateChange", status: null },
            { type: "textMessage", text: "{TARGET} is back to normal!" },
        ]
    },
    item_recoverStatus: {
        name: "Heating Lamp",
        description: "removes user's pizza status",
        targetType: "friendly",
        success: [
            { type: "textMessage", text: "{CASTER} uses a {ACTION}!" },
            { type: "stateChange", status: null },
            { type: "textMessage", text: "Feeling fresh!" },
        ]
    },
    item_recoverHp: {
        name: "Cheese",
        description: "recovers 10 hp",
        targetType: "friendly",
        success: [
            { type: "textMessage", text: "{CASTER} uses a {ACTION}!" },
            { type: "stateChange", recover: 10, },
            { type: "textMessage", text: "recovers Hp!" },
        ]
    },
}