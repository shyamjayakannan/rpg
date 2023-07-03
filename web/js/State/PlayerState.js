class playerState {
    constructor() {
        this.pizzas = {};
        this.lineup = [];
        this.items = [];
    }

    swapLineup(oldId, incomingId) {
        const oldIndex = this.lineup.indexOf(oldId);
        this.lineup[oldIndex] = incomingId;

        document.dispatchEvent(new CustomEvent("LineupStateUpdated"));
    }

    moveToFront(futureFrontId) {
        this.lineup = this.lineup.filter(id => id !== futureFrontId);
        this.lineup.unshift(futureFrontId);
        document.dispatchEvent(new CustomEvent("LineupStateUpdated"));
    }

    addPizza(pizzaId) {
        const newId = `p${Date.now()}`;
        this.pizzas[newId] = {
            pizzaId,
            maxHp: 50,
            xp: 0,
            maxXp: 100,
            level: 1,
            status: null,
        };

        if (this.lineup.length < 3) {
            this.lineup.push(newId);
            
            document.dispatchEvent(new CustomEvent("LineupStateUpdated"));
            return true;
        } else return false;
    }

    addItem(itemType) {
        let already = 0;

        this.items.forEach(item => {
            if (itemType === item.actionId) already += 1;
        });
        this.items.push({ actionId: itemType, instanceId: `${already}` });
    }
}

export const PlayerState = new playerState();