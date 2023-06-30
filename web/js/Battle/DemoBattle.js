import { Combatant } from "./Combatant";
import { Pizzas } from "../Content/pizzas";
import { BattleEvent } from "./BattleEvent";
import { Team } from "./Team";
import { PlayerState } from "../State/PlayerState";
import { OverworldEvent } from "../OverWorldEvent";

export class DemoBattle {
    constructor({ enemy, onComplete }) {
        this.enemy = enemy;
        this.onComplete = onComplete;
        this.combatants = {};

        this.activeCombatants = {
            player: null,
            enemy: null,
        };

        // add player team
        PlayerState.lineup.forEach(id => this.addCombatant(id, "player", PlayerState.pizzas[id]));

        // add enemy team
        Object.keys(this.enemy.pizzas).forEach(key => this.addCombatant("e_" + key, "enemy", this.enemy.pizzas[key]));

        this.items = [];
        PlayerState.items.forEach(item => {
            this.items.push({
                ...item,
                team: "player",
            });
        });

        this.usedInstanceIds = {};
    }

    addCombatant(id, team, config) {
        this.combatants[id] = new Combatant({
            ...Pizzas[config.pizzaId],
            ...config,
            team,
            isPlayerControlled: team === "player",
        }, this);

        this.activeCombatants[team] = this.activeCombatants[team] || id;
    }

    createElement() {
        this.element = document.createElement("div");
        this.element.classList.add("Battle");
        this.element.innerHTML = `
            <div class="Battle_hero">
                <img src="${'images/characters/people/hero.png'}" alt="Hero" />
            </div>
            <div class="Battle_enemy">
                <img src="${this.enemy.src}" alt="${this.enemy.name}" />
            </div>
        `;
    }

    onNewEvent(event) {
        return new Promise(resolve => {
            const battleEvent = new BattleEvent(event, this);
            resolve(battleEvent.init());
        });
    }

    async submissionEvent(currentTeam, swap = false) {
        const casterId = this.activeCombatants[currentTeam];
        const caster = this.combatants[casterId];
        const enemyId = this.activeCombatants[caster.team === "player" ? "enemy" : "player"];
        const enemy = this.combatants[enemyId];

        const submission = await this.onNewEvent({
            type: "submissionMenu",
            caster,
            enemy,
            swap: swap ? true : false,
        });

        // quit
        if (submission === "quit") {
            for (let i = 0; i < this.enemy.message.quit.length; i++) {
                await new OverworldEvent({ event: this.enemy.message.quit[i] }).init()
            }

            this.onComplete();
            return;
        }

        // for replacing pizza
        if (submission.replacement) {
            await this.onNewEvent({
                type: "replace",
                replacement: submission.replacement,
            });

            await this.onNewEvent({
                type: "textMessage", speaker: "Erio",
                text: `yay ${submission.replacement.name}`,
            });

            return;
        }

        if (submission.instanceId) {
            // persist player state
            this.usedInstanceIds[submission.instanceId] = true;

            // remove from battle state
            this.items = this.items.filter(i => i.instanceId !== submission.instanceId);
        }

        const resultingEvents = caster.getReplacedEvents(submission.action.success);
        for (let i = 0; i < resultingEvents.length; i++) {
            await this.onNewEvent({
                ...resultingEvents[i],
                submission,
                action: submission.action,
                target: submission.target,
                caster,
            });
        }

        const postEvents = caster.getPostEvents();
        for (let i = 0; i < postEvents.length; i++) {
            await this.onNewEvent({
                ...postEvents[i],
                submission,
                action: submission.action,
                target: submission.target,
                caster,
            });
        }

        // status expire check
        const expiredEvent = caster.decrementStatus();
        if (expiredEvent) await this.onNewEvent(expiredEvent);
    }

    async init(container) {
        this.createElement();
        container.appendChild(this.element);

        this.playerTeam = new Team("player", "Hero");
        this.enemyTeam = new Team("enemy", "jo");

        Object.keys(this.combatants).forEach(key => {
            let combatant = this.combatants[key];
            combatant.id = key;

            combatant.init(this.element);

            // add correct team
            if (combatant.team === "player") this.playerTeam.combatants.push(combatant);
            else if (combatant.team === "enemy") this.enemyTeam.combatants.push(combatant);
        });

        this.playerTeam.init(this.element);
        this.enemyTeam.init(this.element);

        await this.onNewEvent({
            type: "textMessage", speaker: "Erio",
            text: "Let's begin! Each pizza has an hp(health), xp(experience) and a level. Battles help you gain xp and level up your pizzas",
        });
        await this.onNewEvent({
            type: "textMessage", speaker: "Erio",
            text: "During battles, your pizzas may lose hp. But don't worry! At the end of each battle, all your pizzas will get their hp restored.",
        });
        await this.onNewEvent({
            type: "textMessage", speaker: "Erio",
            text: "The tokens on the top left show how many pizzas you have in your lineup and whether they are active or not. The ones on the right show the same, but for me.",
        });
        await this.onNewEvent({
            type: "textMessage", speaker: "Erio",
            text: "Alright! Try to attack my pizza. press ENTER, then choose Attack from the menu that appears",
        });
        await this.submissionEvent("player");
        await this.onNewEvent({
            type: "textMessage", speaker: "Erio",
            text: "Great first attack! Now I'll make a move.",
        });
        await this.submissionEvent("enemy");
        await this.onNewEvent({
            type: "textMessage", speaker: "Erio",
            text: "Alright! Now try using the item you picked.",
        });
        await this.submissionEvent("player");
        await this.onNewEvent({
            type: "textMessage", speaker: "Erio",
            text: "Wonderful! Note that items once used, will disappear from your inventory. So use them wisely!",
        });
        await this.onNewEvent({
            type: "textMessage", speaker: "Erio",
            text: "My turn again!",
        });
        await this.submissionEvent("enemy", "swap");
        await this.onNewEvent({
            type: "textMessage", speaker: "Erio",
            text: "I just swapped my pizza for another one. You will be able to do it too, once you have more pizzas.",
        });
        await this.submissionEvent("enemy");
        await this.onNewEvent({
            type: "textMessage", speaker: "Erio",
            text: "Different pizzas allow different moves. This pizza has a 'saucy' status. Now, try to attack me.",
        });
        await this.submissionEvent("player");
        await this.submissionEvent("enemy");
        await this.onNewEvent({
            type: "textMessage", speaker: "Erio",
            text: "Saw that? the 'saucy' status will recover some hp after every turn of mine. Attack me once more to see it.",
        });
        await this.submissionEvent("player");
        await this.submissionEvent("enemy");
        await this.onNewEvent({
            type: "textMessage", speaker: "Erio",
            text: "Ok! Let's end the battle now. I hope you've understood how to go about it!",
        });
    
        this.element.remove();
        this.onComplete();
    }
}