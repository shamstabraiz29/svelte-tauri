import { browser } from "@wdio/globals";
import assert from "assert";

import { unloadModelScript } from "./scripts/index.ts";

export default async (): Promise<undefined> => {
    const loadModelRes = await browser.executeAsync(unloadModelScript);

    if (typeof loadModelRes == "string") {
        assert.fail(loadModelRes);
    }

    console.log("Unloaded open model: ");
};
