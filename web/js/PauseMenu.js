import { KeyPressListener } from "./KeyPressListener";
import { KeyboardMenu } from "./KeyboardMenu";
import { PlayerState } from "./State/PlayerState";
import { Pizzas } from "./Content/pizzas";

export class PauseMenu {
    constructor({ progress = null, onComplete, isBattle = false, endBattle = null }) {
        this.onComplete = onComplete;
        this.isBattle = isBattle;
        this.endBattle = endBattle;
        this.progress = progress;
    }

    getOptions(pageKey) {
        if (this.isBattle) {
            return [
                {
                    label: "Quit Battle",
                    description: "Your Battle progress will be lost",
                    handler: () => {
                        this.close();
                        this.endBattle();
                    },
                },
                {
                    label: "Close",
                    description: "Close Menu",
                    handler: () => this.close(),
                },
            ];
        }

        if (pageKey === "root") {
            const lineupPizzas = PlayerState.lineup.map(id => {
                const { pizzaId } = PlayerState.pizzas[id];
                const base = Pizzas[pizzaId];

                return {
                    label: base.name,
                    description: base.description,
                    handler: () => this.keyboardMenu.setOptions(this.getOptions(id)),
                };
            });

            return [
                ...lineupPizzas,
                {
                    label: "Save",
                    description: "Save progress",
                    handler: () => {
                        this.progress.save();
                        this.close();
                    },
                },
                {
                    label: "Close",
                    description: "Close Menu",
                    handler: () => this.close(),
                },
            ];
        }

        const unequipped = Object.keys(PlayerState.pizzas).filter(id => PlayerState.lineup.indexOf(id) === -1).map(id => {
            const {pizzaId} = PlayerState.pizzas[id];
            const base = Pizzas[pizzaId];

            return {
                label: `Swap for ${base.name}`,
                description: base.description,
                handler: () => {
                    PlayerState.swapLineup(pageKey, id);
                    this.keyboardMenu.setOptions(this.getOptions("root"));
                },
            };
        });

        return [
            ...unequipped,
            {
                label: "Move to front",
                description: "Move this pizza to the front",
                handler: () => {
                    PlayerState.moveToFront(pageKey);
                    this.keyboardMenu.setOptions(this.getOptions("root"));
                },
            },
            {
                label: "Back",
                description: "Back to Pause Menu",
                handler: () => this.keyboardMenu.setOptions(this.getOptions("root")),
            },
        ];
    }

    createElement() {
        this.element = document.createElement('div');
        this.element.classList.add("overlayMenu");
        this.element.innerHTML = `<h2>Pause Menu</h2>`;
    }

    close() {
        if (this.esc) this.esc.unbind();
        if (this.keyboardMenu) this.keyboardMenu.end();
        this.element.remove();
        this.onComplete();
    }

    async init(container) {
        this.createElement();

        this.keyboardMenu = new KeyboardMenu({
            descriptionContainer: container,
        });
        this.keyboardMenu.init(this.element);
        this.keyboardMenu.setOptions(this.getOptions("root"));

        container.appendChild(this.element);

        await new Promise(resolve => setTimeout(resolve, 200));
        this.esc = new KeyPressListener("Escape", () => this.close());
    }
}