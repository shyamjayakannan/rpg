export class Combatant {
    constructor(config, battle) {
        Object.keys(config).forEach(key => this[key] = config[key]);
        this.hp = typeof(this.hp) === "undefined" ? this.maxHp : this.hp;
        this.battle = battle;
    }

    createElement() {
        this.hudElement = document.createElement("div");
        this.hudElement.classList.add("Combatant");
        this.hudElement.setAttribute("data-combatant", this.id);
        this.hudElement.setAttribute("data-team", this.team);
        this.hudElement.innerHTML = `
            <p class="Combatant_name">${this.name}</p>
            <p class="Combatant_level"></p>
            <div class="Combatant_character_crop">
                <img src="${this.src}" alt="${this.name}" class="Combatant_character">
            </div>
            <img src="${this.icon}" alt="${this.type}" class="Combatant_type">
            <svg viewBox="0 0 26 3" class="Combatant_life-container">
                <rect x=0 y=0 width="0%" height=1 fill="#b2ff71"/>
                <rect x=0 y=1 width="0%" height=2 fill="#3ef126"/>
            </svg>
            <svg viewBox="0 0 26 2" class="Combatant_xp-container">
                <rect x=0 y=0 width="0%" height=1 fill="#ffd76a"/>
                <rect x=0 y=1 width="0%" height=1 fill="#ffc934"/>
            </svg>
            <p class="Combatant_status"></p>
        `;

        this.pizzaElement = document.createElement('img');
        this.pizzaElement.classList.add("Pizza");
        this.pizzaElement.setAttribute("src", this.src);
        this.pizzaElement.setAttribute("alt", this.name);
        this.pizzaElement.setAttribute("data-team", this.team);

        this.hpFills = this.hudElement.querySelectorAll(".Combatant_life-container > rect");
        this.xpFills = this.hudElement.querySelectorAll(".Combatant_xp-container > rect");
    }

    get hpPercent() {
        const percent = this.hp / this.maxHp * 100;
        return percent > 0 ? percent : 0;
    }

    get xpPercent() {
        return this.xp / this.maxXp * 100;
    }

    get isActive() {
        if (this.battle) return this.battle.activeCombatants[this.team] === this.id;
    }

    get givesXp() {
        return this.level * 20;
    }

    update(changes = {}) {
        Object.keys(changes).forEach(key => this[key] = changes[key]);

        this.pizzaElement.setAttribute("data-active", this.isActive);
        this.hudElement.setAttribute("data-active", this.isActive);
        
        this.hpFills.forEach(rect => rect.style.width = `${this.hpPercent}%`);
        this.xpFills.forEach(rect => rect.style.width = `${this.xpPercent}%`);

        this.hudElement.querySelector(".Combatant_level").innerText = this.level;

        const statusElement = this.hudElement.querySelector(".Combatant_status");
        if (this.status) {
            statusElement.innerText = this.status.type;
            statusElement.style.display = "block";
        } else {
            statusElement.innerText = "";
            statusElement.style.display = "none";
        }
    }

    getPostEvents() {
        if (this.status) {
            if (this.status.type === "saucy") {
                return [
                    { type: "textMessage", text: "Feelin' saucy" },
                    { type: "stateChange", recover: 3, onCaster: true },
                ];
            }
        }

        return [];
    }

    getReplacedEvents(originalEvents) {
        if (this.status) {
            if (this.status.type === "clumsy" && randomFromArray([true, false, false])) {
                return [
                    { type: "textMessage", text: `${this.name} flops over` },
                ];
            }
        }

        return originalEvents;
    }

    decrementStatus() {
        if (this.status && this.status.expiresIn > 0) {
            this.status.expiresIn -= 1;
            if (this.status.expiresIn === 0) {
                const type = this.status.type;
                this.update({
                    status: null,
                });

                return { type: "textMessage", text: `No Longer ${type}!` };
            }
        }

        return null;
    }

    init(container) {
        this.createElement();
        container.appendChild(this.hudElement);
        container.appendChild(this.pizzaElement);
        this.update();
    }
}

export function randomFromArray(array) {
    return array[Math.floor(Math.random() * array.length)];
}