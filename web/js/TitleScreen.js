import { KeyboardMenu } from "./KeyboardMenu";

export class TitleScreen {
    constructor({ progress }) {
        this.progress = progress;
    }

    getOptions(resolve) {
        const saveFile = this.progress.getSaveFile();

        return [
            {
                label: "New Game",
                description: "Start a new adventure!",
                handler: () => {
                    this.close();
                    resolve();
                },
            },
            saveFile ? {
                label: "Continue Game",
                description: "Resume adventure!",
                handler: () => {
                    this.close();
                    resolve(saveFile);
                },
            } : null,
        ].filter(v => v);

        // filter to remove null
    }

    createElement() {
        this.element = document.createElement('div');
        this.element.classList.add("TitleScreen");
        this.element.innerHTML = `<img src="images/logo.png" alt="Pizza Legends" class="TitleScreen_logo">`;
    }

    close() {
        this.keyboardMenu.end();
        this.element.remove();
    }

    init(container) {
        return new Promise(resolve => {
            this.createElement();
            container.appendChild(this.element);
            this.keyboardMenu = new KeyboardMenu();
            this.keyboardMenu.init(this.element);
            this.keyboardMenu.setOptions(this.getOptions(resolve));
        });
    }
}