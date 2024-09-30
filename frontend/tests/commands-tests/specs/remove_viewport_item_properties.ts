import { browser } from "@wdio/globals";
import assert from "assert";

import { viewportItemRemovePropertiesScript } from "./scripts/index.ts";

export default async (
    viewportId: string,
    viewportItemId: string,
    properties: string[]
): Promise<void> => {
    let removeBranchPropertiesRes = await browser.executeAsync(
        viewportItemRemovePropertiesScript,
        viewportItemId,
        properties
    );

    if (typeof removeBranchPropertiesRes === "string") {
        assert.fail(removeBranchPropertiesRes);
    }

    let targetViewportEntry = removeBranchPropertiesRes[viewportId];

    if (!targetViewportEntry) {
        assert.fail(
            "An entry was not found for the target viewport in the response!"
        );
    }

    let targetViewportItem = targetViewportEntry.find((viewportItem) => {
        return viewportItem.id === viewportItemId;
    });

    if (!targetViewportItem) {
        assert.fail(
            "An entry was not found for the target viewport in the response!"
        );
    }

    for (let prop_key of properties) {
        assert.equal(targetViewportItem.properties[prop_key], undefined);
    }
};
