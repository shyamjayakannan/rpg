import { OverWorld } from "../pkg";
import { FIREBASE } from "./FireBase";
import { Hud } from "./js/Hud";
import { KeyPressListener } from "./js/KeyPressListener";
import { OverworldEvent } from "./js/OverWorldEvent";
import { Progress } from "./js/Progress";
import { TitleScreen } from "./js/TitleScreen";

const Directions = {
    "ArrowDown": 0,
    "KeyS": 0,
    "ArrowLeft": 1,
    "KeyA": 1,
    "ArrowRight": 2,
    "KeyD": 2,
    "ArrowUp": 3,
    "KeyW": 3,
};

class OverWorldJS {
    async init() {
        this.overWorld = OverWorld.new(document.querySelector(".game-canvas"), await FIREBASE.getMap("DemoRoom"));
        
        this.progress = new Progress(this.overWorld);

        // title screen
        this.titleScreen = new TitleScreen({ progress: this.progress });
        const savedFile = await this.titleScreen.init(document.querySelector(".game-container"));

        // load saved data
        if (savedFile) await this.progress.load();
        
        this.hud = new Hud();
        this.hud.init(document.querySelector(".game-container"));

        this.bindActionInput();
        this.checkHeroCutscene();

        this.render();
    }

    bindActionInput() {
        new KeyPressListener("Enter", () => {
            let scenes = this.overWorld.check_for_action_cutscene();

            if (scenes && !this.overWorld.is_cutscene_playing()) this.startCutscene(scenes.map(value => Object.fromEntries(value)));
        });

        new KeyPressListener("Escape", () => {
            if (!this.overWorld.is_cutscene_playing() && this.overWorld.is_on_square()) {
                this.startCutscene([
                    { type: "pause", repeat: 1 }
                ]);
            }
        });
    }
    
    checkHeroCutscene() {
        const keyDown = e => {
            if (Directions[e.code] !== undefined) this.overWorld.add_direction(Directions[e.code]);
        }

        const keyUp = e => {
            if (Directions[e.code] !== undefined) this.overWorld.remove_direction(Directions[e.code]);
        }

        document.addEventListener("keydown", keyDown);
        document.addEventListener("keyup", keyUp);
        
        document.addEventListener("HeroWalkingComplete", () => {
            setTimeout(() => {
                this.check_for_action_cutscene();
            }, 10);

            // settimeout to prevent clashes between different eventlisteners trying to mutate overworld as rust does not allow it.
        });
    }

    check_for_action_cutscene() {
        let scenes = this.overWorld.check_for_action();

        if (scenes && !this.overWorld.is_cutscene_playing()) this.startCutscene(scenes.map(value => Object.fromEntries(value)));
    }

    render() {
        this.overWorld.draw();
        this.overWorld.update();
        requestAnimationFrame(() => this.render());
    }

    async startCutscene(events) {
        this.overWorld.start_cutscene();

        for (let i = 0; i < events.length; i++) {
            for (let j = 0; j < events[i].repeat; j++) {
                const eventHandler = new OverworldEvent({
                    event: events[i],
                    overWorldJS: this,
                });

                if (await eventHandler.init() === "end") {
                    this.overWorld.stop_cutscene();
                    this.check_for_action_cutscene();
                    return;
                }
            }
        }

        this.overWorld.stop_cutscene();
        this.check_for_action_cutscene();
    }
}

const overWorldJS = new OverWorldJS();
overWorldJS.init();