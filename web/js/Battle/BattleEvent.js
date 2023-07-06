import { BattleAnimations } from "../Content/BattleAnimations";
import { Pizzas } from "../Content/pizzas";
import { TextMessage } from "../TextMessage";
import { ReplacementMenu } from "./ReplacementMenu";
import { SubmissionMenu } from "./SubmissionMenu";

export class BattleEvent {
    constructor(event, battle) {
        this.battle = battle;
        this.event = event;
    }

    textMessage(resolve) {
        let text = this.event.text;

        if (this.event.caster) {
            text = text
                .replace("{CASTER}", this.event.caster.name)
                .replace("{TARGET}", this.event.target.name)
                .replace("{ACTION}", this.event.action.name)
        }

        const message = new TextMessage({
            text,
            speaker: this.event.speaker,
            onComplete: resolve,
        });

        message.init(this.battle.element);
    }

    submissionMenu(resolve) {
        const { caster } = this.event;

        const menu = new SubmissionMenu({
            caster,
            enemy: this.event.enemy,
            items: this.battle.items,
            replacements: Object.values(this.battle.combatants).filter(c => c.id !== caster.id && c.team === caster.team && c.hp > 0),
            onComplete: submission => resolve(submission),
            endBattle: () => this.battle.element.remove(),
            swap: this.event.swap,
        });

        menu.init(this.battle.element);
    }

    async stateChange(resolve) {
        const { caster, target, damage, status, recover, rebound } = this.event;
        let who = this.event.onCaster ? caster : target;

        if (rebound) {
            this.event = { type: "textMessage", text: "Attack rebounds!" };
            await this.init();
        }
        
        if (damage) {
            // deccrease hp
            who.update({
                hp: who.hp - damage,
            });

            // start blinking
            who.pizzaElement.classList.add("battle-damage-blink");
        }

        if (recover) {
            let newHp = who.hp + recover;
            if (newHp > who.maxHp) newHp = who.maxHp;

            who.update({
                hp: newHp,
            });
        }

        if (status) {
            who.update({
                status: { ...status },
            });
        }

        // wait
        await new Promise(resolve => setTimeout(resolve, 600));

        // update team elements
        this.battle.playerTeam.update();
        this.battle.enemyTeam.update();

        // stop blinking
        who.pizzaElement.classList.remove("battle-damage-blink");

        resolve();
    }

    replacementMenu(resolve) {
        const menu = new ReplacementMenu({
            replacements: Object.values(this.battle.combatants).filter(c => c.team === this.event.team && c.hp > 0),
            onComplete: replacement => resolve(replacement),
        });

        menu.init(this.battle.element);
    }

    giveXp(resolve) {
        let amount = this.event.xp;
        const { combatant } = this.event;

        const step = async () => {
            if (amount > 0) {
                amount -= 1;
                combatant.xp += 1;

                // check for level 
                if (combatant.xp === combatant.maxXp && combatant.level !== 3) {
                    combatant.xp = 0;
                    combatant.maxXp = 100;
                    combatant.level += 1;

                    const pizzaId = combatant.pizzaId;
                    combatant.pizzaId = `${combatant.pizzaId.slice(0, -1)}${parseInt(pizzaId.slice(-1)) + 1}`;

                    combatant.update({ ...Pizzas[combatant.pizzaId] });
                    combatant.init(this.battle.element);

                    this.event = { type: "textMessage", text: `${Pizzas[pizzaId].name} upgraded to ${combatant.name}` };
                    await this.init();
                }

                combatant.update();
                requestAnimationFrame(step);
                return;
            }

            resolve();
        };

        step();
    }

    async replace(resolve) {
        const { replacement } = this.event;

        // remove old
        const prev = this.battle.activeCombatants[replacement.team];
        this.battle.activeCombatants[replacement.team] = null;
        this.battle.combatants[prev].update();

        await new Promise(resolve => setTimeout(resolve, 400));

        // put new
        this.battle.activeCombatants[replacement.team] = replacement.id;
        replacement.update();

        await new Promise(resolve => setTimeout(resolve, 400));

        // update team elements
        this.battle.playerTeam.update();
        this.battle.enemyTeam.update();

        resolve();
    }

    animation(resolve) {
        BattleAnimations[this.event.animation](this.event, resolve);
    }

    init() {
        return new Promise(resolve => {
            this[this.event.type](resolve);
        })
    }
}
