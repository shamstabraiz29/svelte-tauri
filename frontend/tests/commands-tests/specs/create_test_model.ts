import { browser } from "@wdio/globals";
import assert from "assert";

import { createTestModelScript } from "./scripts/index.ts";
import { ViewportItemMeta } from "tauri-plugin-editor";

export default async (
    viewportId: string
): Promise<{ [x: string]: ViewportItemMeta[] }> => {
    const loadModelRes = await browser.executeAsync(
        createTestModelScript,
        viewportId
    );

    if (typeof loadModelRes == "string") {
        assert.fail(loadModelRes);
    }

    console.log("Generated test model. Created Items: ");
    console.log(JSON.stringify(loadModelRes, null, 2));

    return loadModelRes;
};
