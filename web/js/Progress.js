import { FIREBASE } from "../FireBase";
import { PlayerState } from "./State/PlayerState";

export class Progress {
    constructor(overWorld) {
        this.saveFileKey = "PizzaLegends_SaveFile1";
        this.overWorld = overWorld;
    }

    save() {
        window.localStorage.setItem(this.saveFileKey, JSON.stringify({
            mapName: this.overWorld.get_map_name(),
            playerState: {
                pizzas: PlayerState.pizzas,
                lineup: PlayerState.lineup,
                items: PlayerState.items,
                storyFlags: this.overWorld.get_story_flags(),
            },
        }));
    }

    getSaveFile() {
        const file = window.localStorage.getItem(this.saveFileKey);
        return file ? JSON.parse(file) : null;
    }

    async load() {
        const file = this.getSaveFile();

        if (file) {
            this.overWorld.set_story_flags_from_progress(file.playerState.storyFlags);
            this.overWorld.change_map("down", "", await FIREBASE.getMap(file.mapName));

            Object.keys(file.playerState).forEach(key => PlayerState[key] = file.playerState[key]);
        }
    }
}