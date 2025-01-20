const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

function corelog(text) {
    invoke("log", { "text": text });
}

function add_component(html) {
    const groupContainer = document.querySelector(".scrollable");
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, "text/html");
    const group = doc.body.firstChild;

    if (group) {
        groupContainer.appendChild(group);
    }
}

document.addEventListener("DOMContentLoaded", () => {
    corelog("DOMContentLoaded")

    invoke("get_default_components")
        .then((data) => {
            data.forEach(x => add_component(x));
        })
        .then(() => {
            const buttons = document.querySelectorAll(".group-content button");
            buttons.forEach((button) =>
                button.addEventListener("click", () => {
                    // On clicked event
                })
            );
        })
        .catch(e => console.log(JSON.stringify(e)))
});
