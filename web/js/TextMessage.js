import { KeyPressListener } from "./KeyPressListener";
import { RevealingText } from "./RevealingText";

export class TextMessage {
    constructor({ text, onComplete, speaker }) {
        this.text = text;
        this.speaker = speaker;
        this.onComplete = onComplete;
        this.element = null;
    }

    createElement() {
        this.element = document.createElement('div');
        this.element.classList.add("TextMessage");

        this.element.innerHTML = `
            <h2 class="TextMessage_h2">${this.speaker || ""}</h2>
            <p class="TextMessage_p"></p>
            <button class="TextMessage_button">Next</button>
        `;

        this.revealingText = new RevealingText({
            element: this.element.querySelector(".TextMessage_p"),
            text: this.text,
        });

        this.actionListener = new KeyPressListener("Enter", () => this.done());
    }
    
    done() {
        if (this.revealingText.isDone) {
            this.actionListener.unbind();
            this.element.remove();

            document.removeEventListener("click", () => this.done());

            this.onComplete();
        } else {
            this.revealingText.warpToDone();
        }
    }
    
    init(container) {
        this.createElement();
        container.appendChild(this.element);
        document.querySelector(".TextMessage_button").addEventListener("click", () => this.done());
        this.revealingText.init();
    }
}