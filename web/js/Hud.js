import { PlayerState } from "./State/PlayerState";
import { Combatant } from "./Battle/Combatant";
import { Pizzas } from "./Content/pizzas";

export class Hud {
    constructor() {
        this.scoreboards = [];
    }

    update() {
        this.scoreboards.forEach(s => s.update(PlayerState.pizzas[s.id]));
    }

    createElement() {
        if (this.element) {
            this.element.remove();
            this.scoreboards = [];
        }

        this.element = document.createElement('div');
        this.element.classList.add("Hud");

        PlayerState.lineup.forEach(key => {
            const pizza = PlayerState.pizzas[key];
            const scoreboard = new Combatant({
                id: key,
                ...Pizzas[pizza.pizzaId],
                ...pizza,
            }, null);

            scoreboard.createElement();
            this.scoreboards.push(scoreboard);
            this.element.appendChild(scoreboard.hudElement);
        });

        this.update();
    }

    init(container) {
        this.createElement();
        container.appendChild(this.element);

        document.addEventListener("PlayerStateUpdated", () => this.update());
        document.addEventListener("LineupStateUpdated", () => {
            this.createElement();
            container.appendChild(this.element);
        });
    }
}