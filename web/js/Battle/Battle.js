import { Combatant } from "./Combatant";
import { Pizzas } from "../Content/pizzas";
import { TurnCycle } from "./TurnCycle";
import { BattleEvent } from "./BattleEvent";
import { Team } from "./Team";
import { PlayerState } from "../State/PlayerState";
import { OverworldEvent } from "../OverWorldEvent";

export class Battle {
    constructor({ enemy, onComplete, background }) {
        this.enemy = enemy;
        this.onComplete = onComplete;
        this.combatants = {};
        this.background = background;

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

    init(container) {
        this.createElement();
        container.appendChild(this.element);

        // background
        document.querySelector(".Battle").style.backgroundImage = `url(${this.background})`;

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

        this.turnCycle = new TurnCycle({
            battle: this,
            onNewEvent: event => {
                return new Promise(resolve => {
                    const battleEvent = new BattleEvent(event, this);
                    resolve(battleEvent.init());
                });
            },
            onWinner: async (winner, loser) => {
                if (winner === "player") {
                    Object.keys(PlayerState.pizzas).forEach(id => {
                        const playerStatePizza = PlayerState.pizzas[id];
                        const combatant = this.combatants[id];

                        if (combatant) {
                            playerStatePizza.hp = combatant.hp;
                            playerStatePizza.xp = combatant.xp;
                            playerStatePizza.maxXp = combatant.maxXp;
                            playerStatePizza.level = combatant.level;
                        }
                    });

                    PlayerState.items = PlayerState.items.filter(item => !this.usedInstanceIds[item.instanceId]);

                    // send signal to HUD
                    document.dispatchEvent(new CustomEvent("PlayerStateUpdated"));

                    this.element.remove();

                    for (let i = 0; i < loser.message.lost.length; i++) {
                        await new OverworldEvent({ event: loser.message.lost[i] }).init()
                    }
                } else {
                    this.element.remove();
                    
                    // winner is enemy
                    for (let i = 0; i < winner.message.won.length; i++) {
                        await new OverworldEvent({ event: winner.message.won[i] }).init()
                    }
                }

                this.onComplete();
            },
        });

        this.turnCycle.init();
    }
}