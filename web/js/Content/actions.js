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
    saucyStatus: {
        name: "Tomato Squeeze!",
        targetType: "friendly",
        description: "Gives user saucy status for 3 turns",
        success: [
            { type: "textMessage", text: "{CASTER} uses {ACTION}!" },
            { type: "stateChange", status: { type: "saucy", expiresIn: 3 } },
        ]
    },
    clumsyStatus: {
        name: "Olive Oil!",
        description: "Increases the probability of missing attacks for the target",
        success: [
            { type: "textMessage", text: "{CASTER} uses {ACTION}!" },
            { type: "animation", animation: "glob", color: "#dafd2a" },
            { type: "stateChange", status: { type: "clumsy", expiresIn: 3 } },
            { type: "textMessage", text: "{TARGET} is slipping!" },
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