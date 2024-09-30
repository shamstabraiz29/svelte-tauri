import { browser } from "@wdio/globals";
import assert from "assert";

import { viewportUpsertPropertiesScript } from "./scripts/index.ts";

export default async (
    viewportId: string,
    properties: { [x: string]: any }
): Promise<void> => {
    let upsertViewportPropertiesRes = await browser.executeAsync(
        viewportUpsertPropertiesScript,
        viewportId,
        properties
    );

    if (typeof upsertViewportPropertiesRes === "string") {
        assert.fail(upsertViewportPropertiesRes);
    }

    let targetViewport = upsertViewportPropertiesRes[viewportId] || null;

    if (!targetViewport) {
        assert.fail("The target viewport item was not found in the model!");
    }

    for (let prop_key in properties) {
        assert.equal(targetViewport.properties[prop_key], properties[prop_key]);
    }
};
