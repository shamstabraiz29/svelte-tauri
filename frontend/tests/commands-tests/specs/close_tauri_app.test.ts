import { browser } from "@wdio/globals";

import { exit } from "@tauri-apps/plugin-process";

declare var describe: any;
declare var it: any;

describe("Closes the Tauri application", () => {
    it("close tauri application", async () => {
        browser.executeAsync(function (done) {
            exit(0)
                .then((response) => done(response))
                .catch((error) => done(error));
        });
    });
});
