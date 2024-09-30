import { browser } from "@wdio/globals";
import assert from "assert";

import { viewportItemUpsertPropertiesScript } from "./scripts/index.ts";

export default async (
    viewportId: string,
    viewportItemId: string,
    properties: { [x: string]: any }
): Promise<void> => {
    let upsertBranchPropertiesRes = await browser.executeAsync(
        viewportItemUpsertPropertiesScript,
        viewportItemId,
        properties
    );

    if (typeof upsertBranchPropertiesRes === "string") {
        assert.fail(upsertBranchPropertiesRes);
    }

    let targetViewportItem =
        upsertBranchPropertiesRes[viewportId]?.find((vpItem) => {
            return vpItem.id === viewportItemId;
        }) || null;

    if (!targetViewportItem) {
        assert.fail("The target viewport item was not found in the model!");
    }

    for (let prop_key in properties) {
        assert.equal(
            targetViewportItem.properties[prop_key],
            properties[prop_key]
        );
    }
};
