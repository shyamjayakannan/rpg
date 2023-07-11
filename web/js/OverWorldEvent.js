import { Battle } from "./Battle/Battle";
import { Enemies } from "./Content/enemies";
import { PauseMenu } from "./PauseMenu";
import { SceneTransition } from "./SceneTransition";
import { TextMessage } from "./TextMessage";
import { CraftingMenu } from "./CraftingMenu";
import { PlayerState } from "./State/PlayerState";
import { DemoBattle } from "./Battle/DemoBattle";
import { Actions } from "./Content/actions";
import { ReplyMenu } from "./ReplyMenu";
import { FIREBASE } from "../FireBase";

export class OverworldEvent {
    constructor({ event, overWorldJS = undefined }) {
        this.overWorld = overWorldJS ? overWorldJS.overWorld : null;
        this.event = event;
        this.overWorldJS = overWorldJS;
    }

    async addItem(resolve) {
        const { itemType, index } = this.event;
        PlayerState.addItem(itemType);
        
        const action = Actions[this.event.itemType];
        this.event = { type: "textMessage", text: `You found a ${action.name}. Power: ${action.description}.` }
        await this.init();

        this.overWorld.remove_item(index);
        resolve();
    }

    craftingMenu(resolve) {
        const menu = new CraftingMenu({
            pizzas: this.event.pizzas.split(" "),
            onComlpete: text => resolve(text),
            overWorld: this.overWorld,
            stoneId: this.event.index,
        });

        menu.init(document.querySelector(".game-container"));
    }

    async replyMenu(resolve) {
        const menu = new ReplyMenu();
        const event = this.event;

        if (await menu.init(document.querySelector(".game-container")) === "no") {
            for (let i = 0; i < Enemies[event.enemy].message.battleDeclined.length; i++) {
                this.event = Enemies[event.enemy].message.battleDeclined[i];
                await this.init();
            }

            resolve("end");
            return;
        }

        resolve();
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
        sceneTransition.init(document.querySelector(".game-container"), async () => {
            this.overWorld.change_map(this.event.direction, this.event.heroPosition, await FIREBASE.getMap(this.event.map));
            resolve();

            sceneTransition.fadeOut();
        });
    }

    battle(resolve) {
        const battle = new Battle({
            enemy: Enemies[this.event.enemyId],
            onComplete: text => resolve(text),
            background: this.event.background,
            overWorldJS: this.overWorldJS,
        });
        
        battle.init(document.querySelector(".game-container"));
    }
    
    demoBattle(resolve) {
        const battle = new DemoBattle({
            enemy: Enemies[this.event.enemyId],
            onComplete: text => resolve(text),
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
    
    showPizzaStone(resolve) {
        this.overWorld.make_pizza_stone_visible(this.event.index);
        resolve();
    }

    showItem(resolve) {
        this.overWorld.make_item_visible(this.event.index);
        resolve();
    }

    init() {
        return new Promise(resolve => {
            this[this.event.type](resolve)
        })
    }
}
