export {};

import {
    commands as accountCommands,
    events as accountEvents,
} from "tauri-plugin-account-client";
import { commands as loginCommands } from "tauri-plugin-cognito-login";
import { commands as signUpCommands } from "tauri-plugin-cognito-sign-up";
import {
    commands as editorCommands,
    events as editorEvents,
} from "tauri-plugin-editor";
import {
    commands as repoCommands,
    events as repoEvents,
} from "tauri-plugin-repo-client";
import { commands as subscriberCommands } from "tauri-plugin-subscriber-client";

import { exit } from "@tauri-apps/plugin-process";

declare global {
    interface Window {
        subscriberCommands: any;
        loginCommands: any;
        accountCommands: any;
        accountEvents: any;
        signUpCommands: any;
        editorCommands: any;
        editorEvents: any;
        repoCommands: any;
        repoEvents: any;
        exit: any;
    }
}

window.subscriberCommands = subscriberCommands;
window.loginCommands = loginCommands;
window.accountCommands = accountCommands;
window.accountEvents = accountEvents;
window.signUpCommands = signUpCommands;
window.editorCommands = editorCommands;
window.repoCommands = repoCommands;
window.repoEvents = repoEvents;
window.editorEvents = editorEvents;
window.exit = exit;

addEventListener("DOMContentLoaded", () => {
    let tokenInput = document.getElementById("mfaCode") as HTMLInputElement;
    tokenInput.addEventListener("keydown", (event) => {
        if (event.key === "Enter") {
            event.preventDefault();
            if (!tokenInput.value) {
                return;
            }
            let hiddenDiv = document.getElementById(
                "hiddenDiv"
            ) as HTMLDivElement;
            let span = document.createElement("span");
            span.setAttribute("id", "mfaCodeSet");
            hiddenDiv.appendChild(span);
        }
    });
});
