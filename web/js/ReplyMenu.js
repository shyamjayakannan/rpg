import { KeyboardMenu } from "./KeyboardMenu";

export class ReplyMenu {
    constructor() {}

    getPages() {
        return [
            {
                label: "Yes",
                description: "yes",
                handler: () => this.menuSubmit("yes"),
            },
            {
                label: "No",
                description: "no",
                handler: () => this.menuSubmit("no"),
            },
        ];
    }

    menuSubmit(action) {
        if (this.keyboardMenu) this.keyboardMenu.end();

        this.onComplete(action);
    }

    showMenu(container) {
        this.keyboardMenu = new KeyboardMenu();
        this.keyboardMenu.init(container);
        this.keyboardMenu.setOptions(this.getPages());
    }

    async init(container) {
        return new Promise(resolve => {
            this.element = document.createElement('div');
            this.element.classList.add("TextMessage");
            container.appendChild(this.element);
            
            this.onComplete = resolve;
            this.showMenu(this.element);
        });
    }
}