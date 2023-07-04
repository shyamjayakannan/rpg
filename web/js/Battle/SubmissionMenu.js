import { Actions } from "../Content/actions";
import { KeyPressListener } from "../KeyPressListener";
import { KeyboardMenu } from "../KeyboardMenu";
import { PauseMenu } from "../PauseMenu";

export class SubmissionMenu {
    constructor({ caster, enemy, onComplete, items, replacements, endBattle, swap }) {
        this.caster = caster;
        this.enemy = enemy;
        this.onComplete = onComplete;
        this.replacements = replacements;
        this.endBattle = endBattle;
        this.swap = swap;

        let quantityMap = {};
        items.forEach(item => {
            if (item.team === caster.team) {
                let existing = quantityMap[item.actionId];

                if (existing) existing.quantity += 1;
                else {
                    quantityMap[item.actionId] = {
                        actionId: item.actionId,
                        quantity: 1,
                        instanceId: item.instanceId,
                    };
                }
            }
        });

        this.items = Object.values(quantityMap);
    }

    getPages() {
        const backOption = {
            label: "Go Back",
            description: "Go Back",
            handler: () => this.keyboardMenu.setOptions(this.getPages().root),
        };

        return {
            root: [
                {
                    label: "Attack",
                    description: "Choose an attack",
                    handler: () => this.keyboardMenu.setOptions(this.getPages().attacks),
                },
                {
                    label: "Items",
                    description: "Choose an item",
                    handler: () => this.keyboardMenu.setOptions(this.getPages().items),
                },
                {
                    label: "Swap",
                    description: "Choose another pizza",
                    handler: () => this.keyboardMenu.setOptions(this.getPages().replacements),
                },
            ],
            attacks: [
                ...this.caster.actions.map(key => {
                    const action = Actions[key];

                    return {
                        label: action.name,
                        description: action.description,
                        handler: () => this.menuSubmit(action),
                    };
                }),
                backOption,
            ],
            items: [
                ...this.items.map(item => {
                    const action = Actions[item.actionId];

                    return {
                        label: action.name,
                        right: () => "x" + item.quantity,
                        description: action.description,
                        handler: () => this.menuSubmit(action, item.instanceId),
                    };
                }),
                backOption,
            ],
            replacements: [
                ...this.replacements.map(replacement => {
                    return {
                        label: replacement.name,
                        description: replacement.description,
                        handler: () => this.menuSubmitReplacement(replacement),
                    };
                }),
                backOption,
            ],
        };
    }

    menuSubmitReplacement(replacement) {
        if (this.keyboardMenu) this.keyboardMenu.end();
        this.onComplete({ replacement });
    }

    menuSubmit(action, instanceId = null) {
        if (this.keyboardMenu) this.keyboardMenu.end();
        if (this.pause) this.pause.unbind();

        this.onComplete({
            action,
            instanceId,
            target: action.targetType !== "friendly" ? this.enemy : this.caster,
            caster: this.caster,
        });
    }

    decide() {
        if (this.swap) this.menuSubmitReplacement(this.replacements[0]);
        else if (this.enemy.status !== null && this.caster.actions.includes("removeStatus")) this.menuSubmit(Actions.removeStatus);
        else {
            const actions = this.caster.actions.filter(a => a !== "removeStatus");
            this.menuSubmit(Actions[actions[Math.floor(Math.random() * actions.length)]]);
        }
    }

    showMenu(container) {
        this.keyboardMenu = new KeyboardMenu();
        this.keyboardMenu.init(container);
        this.keyboardMenu.setOptions(this.getPages().root);
    }

    init(container) {
        if (this.caster.isPlayerControlled) {
            this.showMenu(container);

            // pause menu allowed
            this.pause = new KeyPressListener("Escape", () => {
                const menu = new PauseMenu({
                    onComplete: () => {},
                    isBattle: true,
                    endBattle: () => {
                        this.pause.unbind();
                        if (this.keyboardMenu) this.keyboardMenu.end();
                        this.endBattle();
                        this.onComplete("quit");
                    },
                });

                menu.init(document.querySelector(".game-container"));
            });
        }
        else this.decide();
    }
}