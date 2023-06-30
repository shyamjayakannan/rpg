export function emit_event(name, who) {
    document.dispatchEvent(new CustomEvent(name, { detail: { who } }));
}