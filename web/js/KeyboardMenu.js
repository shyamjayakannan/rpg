import { KeyPressListener } from "./KeyPressListener";

export class KeyboardMenu {
    constructor(config = {}) {
        this.options = [];
        this.up = null;
        this.down = null;
        this.prevFocus = null;
        this.descriptionContainer = config.descriptionContainer || null;
    }

    setOptions(options) {
        this.options = options;
        this.element.innerHTML = this.options.map((option, index) => {
            const disabledAttr = option.disabled ? "disabled" : "";

            return `
                <div class="option">
                    <button ${disabledAttr} data-button="${index}" data-description="${option.description}">${option.label}</button>
                    <span class="right">${option.right ? option.right() : ""}</span>
                </div>
            `;
        }).join("");

        this.element.querySelectorAll("button").forEach(button => {
            button.addEventListener("click", () => {
                const chosenOption = this.options[Number(button.dataset.button)];
                chosenOption.handler();
            });

            button.addEventListener("mouseenter", () => button.focus());
            button.addEventListener("focus", () => {
                this.prevFocus = button;
                this.descriptionElementText.innerText = button.dataset.description;
            });
        });

        setTimeout(() => this.element.querySelector("button[data-button]:not([disabled])").focus(), 10);
    }

    createElement() {
        this.element = document.createElement('div');
        this.element.classList.add("KeyboardMenu");

        // description box
        this.descriptionElement = document.createElement('div');
        this.descriptionElement.classList.add("DescriptionBox");
        this.descriptionElement.innerHTML = `<p>yay</p>`;
        this.descriptionElementText = this.descriptionElement.querySelector('p');
    }

    end() {
        this.element.remove();
        this.descriptionElement.remove();

        this.up.unbind();
        this.down.unbind();
    }

    init(container) {
        this.createElement();
        (this.descriptionContainer || container).appendChild(this.descriptionElement);
        container.appendChild(this.element);

        this.up = new KeyPressListener("ArrowUp", () => {
            let current = Number(this.prevFocus.getAttribute("data-button"));
            const array = Array.from(this.element.querySelectorAll("button[data-button]"));

            if (current === 0) current = array.length;

            const prevButton = array.reverse().find(el => el.dataset.button < current && !el.disabled);

            if (prevButton) prevButton.focus();
        });

        this.down = new KeyPressListener("ArrowDown", () => {
            let current = Number(this.prevFocus.getAttribute("data-button"));
            const array = Array.from(this.element.querySelectorAll("button[data-button]"));

            if (current === array.length - 1) current = -1;

            const nextButton = array.find(el => el.dataset.button > current && !el.disabled);

            if (nextButton) nextButton.focus();
        });
    }
}