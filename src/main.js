const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

function corelog(text) {
    invoke("log", { "text": text });
}

function add_component(html) {
    const groupContainer = document.getElementById("container");
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, "text/html");
    const group = doc.body.firstChild;

    if (group) {
        groupContainer.appendChild(group);
    }
}

document.addEventListener("DOMContentLoaded", () => {
    invoke("get_default_components")
        .then((data) => {
            data.forEach(x => add_component(x));
        })
        .then(() => {
            const groups = document.querySelectorAll(".group");
            groups.forEach((group) => {
                const button1 = group.querySelector(".button-check");
                const button2 = group.querySelector(".button-exec");
                const textbox = group.querySelector(".textbox");
                const combobox = group.querySelector(".combobox");
                const cmd_id = Number(group.dataset.cmdid);
                if (cmd_id === NaN) {
                    corelog(`Failed to convert ${cmd_id} to number`)
                    return;
                }

                if (button1) {
                    button1.addEventListener("click", () => {
                        invoke("get_registry_value", { "cmdId": cmd_id });
                    });
                }

                if (button2) {
                    button2.addEventListener("click", () => {
                        invoke("set_registry_value", { "cmdId": cmd_id, "value": combobox.value });
                    });
                }
            });
        })
        .catch(e => console.log(JSON.stringify(e)))
});
