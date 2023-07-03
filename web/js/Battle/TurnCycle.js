import { OverworldEvent } from "../OverWorldEvent";

export class TurnCycle {
    constructor({ battle, onNewEvent, onWinner }) {
        this.battle = battle;
        this.onNewEvent = onNewEvent;
        this.currentTeam = "player";
        this.onWinner = onWinner;
    }

    async turn() {
        const casterId = this.battle.activeCombatants[this.currentTeam];
        const caster = this.battle.combatants[casterId];
        const enemyId = this.battle.activeCombatants[caster.team === "player" ? "enemy" : "player"];
        const enemy = this.battle.combatants[enemyId];

        const submission = await this.onNewEvent({
            type: "submissionMenu",
            caster,
            enemy,
        });

        // quit
        if (submission === "quit") {
            for (let i = 0; i < this.battle.enemy.message.quit.length; i++) {
                await new OverworldEvent({ event: this.battle.enemy.message.quit[i] }).init()
            }

            this.battle.onComplete("end");
            return;
        }

        // for replacing pizza
        if (submission.replacement) {
            await this.onNewEvent({
                type: "replace",
                replacement: submission.replacement,
            });

            await this.onNewEvent({
                type: "textMessage",
                text: `${submission.replacement.name} has entered the battle`,
            });

            this.next();
            return;
        }

        if (submission.instanceId) {
            // persist player state
            this.battle.usedInstanceIds[submission.instanceId] = true;

            // remove from battle state
            this.battle.items = this.battle.items.filter(i => i.instanceId !== submission.instanceId);
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

        // target done?
        if (submission.target.hp <= 0) {
            await this.onNewEvent({
                type: "textMessage", text: `${submission.target.name} is done`,
            });

            if (submission.target.team === "enemy") {
                const xp = submission.target.givesXp;

                await this.onNewEvent({
                    type: "textMessage",
                    text: `Gained ${xp} xp`,
                });

                await this.onNewEvent({
                    type: "giveXp",
                    xp: xp,
                    combatant: this.battle.combatants[this.battle.activeCombatants.player],
                });
            }

            // game over?
            const participants = this.getWinningTeam();
            if (participants) {
                await this.onNewEvent({
                    type: "textMessage",
                    text: participants[0] === "player" ? "you won" : `${participants[0].name} won`,
                });

                this.onWinner(participants[0], participants[1]);
                return;
            }

            // not over
            const replacement = await this.onNewEvent({
                type: "replacementMenu",
                team: submission.target.team,
            });

            await this.onNewEvent({
                type: "replace",
                replacement: replacement,
            });

            await this.onNewEvent({
                type: "textMessage",
                text: `yay ${replacement.name}`,
            });
        }

        // post events
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

        this.next();
    }

    next() {
        this.currentTeam = this.currentTeam === "player" ? "enemy" : "player";
        this.turn();
    }

    getWinningTeam() {
        let aliveTeams = {};
        Object.values(this.battle.combatants).forEach(c => {
            if (c.hp > 0) aliveTeams[c.team] = true;
        });

        if (!aliveTeams["player"]) return [this.battle.enemy, "player"];
        if (!aliveTeams["enemy"]) return ["player", this.battle.enemy];
        return null;
    }

    async init() {
        await this.onNewEvent({
            type: "textMessage",
            text: "battle starting",
        });

        this.turn();
    }
}