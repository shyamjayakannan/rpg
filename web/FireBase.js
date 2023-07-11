import { FIREBASE_DOMAIN } from "./Secrets";

class FireBase {
    async getMap(name) {
        try {
            const response = await fetch(`${FIREBASE_DOMAIN}/maps/${name}.json`);
            const resData = await response.json();
    
            if (!response.ok) {
                throw new Error(resData.message || 'Could not fetch user details.');
            }
    
            if (resData.npcs === undefined) resData.npcs = [];

            ["npc_cutscenes", "action_cutscenes"].forEach(type => {
                if (resData[type] === undefined) resData[type] = [];
                else {
                    resData[type].forEach(elem => {
                        elem.scenes.forEach(scene => {
                            if (scene.flags === undefined) scene.flags = []
                        });
                    });
                }
            });
            
            ["pizza_stones", "items"].forEach(type => {
                if (resData[type] === undefined) resData[type] = [];
                else {
                    resData[type].forEach(elem => {
                        if (elem.scene.flags === undefined) elem.scene.flags = [];
                    });
                }
            });

            return resData;
        }
        catch (error) {
            throw new Error(error);
        }
    }
}

export const FIREBASE = new FireBase();