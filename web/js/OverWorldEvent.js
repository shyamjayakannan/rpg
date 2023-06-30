import { Battle } from "./Battle/Battle";
import { Enemies } from "./Content/enemies";
import { PauseMenu } from "./PauseMenu";
import { SceneTransition } from "./SceneTransition";
import { TextMessage } from "./TextMessage";
import { CraftingMenu } from "./CraftingMenu";
import { PlayerState } from "./State/PlayerState";
import { DemoBattle } from "./Battle/DemoBattle";

export class OverworldEvent {
    constructor({ event, overWorldJS = undefined }) {
        this.overWorld = overWorldJS ? overWorldJS.overWorld : null;
        this.event = event;
        this.overWorldJS = overWorldJS;
    }

    addItem(resolve) {
        PlayerState.addItem(this.event.itemType);

        this.overWorld.remove_item(this.event.index);
        resolve();
    }

    craftingMenu(resolve) {
        const menu = new CraftingMenu({
            pizzas: this.event.pizzas.split(" "),
            onComlpete: resolve,
            overWorld: this.overWorld,
            stoneId: this.event.index,
        });

        menu.init(document.querySelector(".game-container"));
    }

    stand(resolve) {
        this.overWorld.stand(this.event.who, this.event.direction);

        //Set up a handler to complete when correct person is done standing, then resolve the event
        const completeHandler = e => {
            if (e.detail.who === this.event.who) {
                document.removeEventListener("Complete", completeHandler);
                resolve();
            }
        }
        document.addEventListener("Complete", completeHandler)
    }

    walk(resolve) {
        this.overWorld.walk(this.event.who, this.event.direction);

        //Set up a handler to complete when correct person is done walking, then resolve the event
        const completeHandler = e => {
            if (e.detail.who === this.event.who) {
                document.removeEventListener("Complete", completeHandler);
                resolve();
            }
        }
        document.addEventListener("Complete", completeHandler)
    }

    textMessage(resolve) {
        const message = new TextMessage({
            text: this.event.text,
            speaker: this.event.speaker,
            onComplete: resolve
        });

        message.init(document.querySelector(".game-container"));
    }

    cameraMove(resolve) {
        this.overWorld.set_camera(this.event.location.split(" "));

        const completeHandler = e => {
            if (e.detail.who === "camera") {
                document.removeEventListener("Complete", completeHandler);
                resolve();
            }
        }
        document.addEventListener("Complete", completeHandler)
    }

    changeMap(resolve) {
        const sceneTransition = new SceneTransition();
        sceneTransition.init(document.querySelector(".game-container"), () => {
            this.overWorld.change_map(this.event.map, this.event.direction);
            resolve();

            sceneTransition.fadeOut();
        });
    }

    battle(resolve) {
        const battle = new Battle({
            enemy: Enemies[this.event.enemyId],
            onComplete: resolve,
            background: this.event.background,
        });
        
        battle.init(document.querySelector(".game-container"));
    }
    
    demoBattle(resolve) {
        const battle = new DemoBattle({
            enemy: Enemies[this.event.enemyId],
            onComplete: resolve,
            background: this.event.background,
        });

        battle.init(document.querySelector(".game-container"));
    }

    pause (resolve) {
        const menu = new PauseMenu({
            progress: this.overWorldJS.progress,
            onComplete: resolve,
        });

        menu.init(document.querySelector(".game-container"));
    }

    addStoryFlag(resolve) {
        this.overWorld.add_story_flag(this.event.flag);
        resolve();
    }

    removeStoryFlag(resolve) {
        this.overWorld.remove_story_flag(this.event.flag);
        resolve();
    }
    
    init() {
        return new Promise(resolve => {
            this[this.event.type](resolve)
        })
    }
}
