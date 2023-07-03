import { Pizzas } from "./Content/pizzas";
import { KeyboardMenu } from "./KeyboardMenu";
import { OverworldEvent } from "./OverWorldEvent";
import { PlayerState } from "./State/PlayerState";

export class CraftingMenu {
    constructor({ pizzas, onComlpete, overWorld, stoneId }) {
        this.pizzas = pizzas;
        this.onComlpete = onComlpete;
        this.overWorld = overWorld;
        this.stoneId = stoneId;
    }

    getOptions() {
        return [
            ...this.pizzas.map(id => {
                const base = Pizzas[id];

                return {
                    label: base.name,
                    description: base.description,
                    handler: async () => {
                        const canAdd = PlayerState.addPizza(id);
                        this.close();

                        if (!canAdd) await new OverworldEvent({ event: { type: "textMessage", text: "Lineup can have at most 3 pizzas" } }).init();
                        else this.overWorld.set_pizza_stone_used(this.stoneId);
                        
                        this.onComlpete();
                    },
                };
            }),
            {
                label: "Close",
                description: "Close Menu",
                handler: () => {
                    this.close();
                    this.onComlpete("end");
                },
            }
        ];
    }

    createElement() {
        this.element = document.createElement('div');
        this.element.classList.add("overlayMenu");
        this.element.innerHTML = `<h2>Create a Pizza</h2>`;
    }

    close() {
        this.keyboardMenu.end();
        this.element.remove();
    }

    init(container) {
        this.createElement();
        this.keyboardMenu = new KeyboardMenu({
            descriptionContainer: container,
        });
        this.keyboardMenu.init(this.element);
        this.keyboardMenu.setOptions(this.getOptions());

        container.appendChild(this.element);
    }
}