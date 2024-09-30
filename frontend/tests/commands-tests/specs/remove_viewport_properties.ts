import { browser } from "@wdio/globals";
import assert from "assert";

import { viewportRemovePropertiesScript } from "./scripts/index.ts";

export default async (
    viewportId: string,
    properties: string[]
): Promise<void> => {
    let removeViewportPropertiesRes = await browser.executeAsync(
        viewportRemovePropertiesScript,
        viewportId,
        properties
    );

    if (typeof removeViewportPropertiesRes === "string") {
        assert.fail(removeViewportPropertiesRes);
    }

    for (let prop_key of properties) {
        assert.equal(removeViewportPropertiesRes[prop_key], undefined);
    }
};
