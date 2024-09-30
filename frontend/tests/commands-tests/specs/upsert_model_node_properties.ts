import { browser } from "@wdio/globals";
import assert from "assert";

import { modelNodeUpsertPropertiesScript } from "./scripts/index.ts";

export default async (
    modelItemId: string,
    properties: { [x: string]: any }
): Promise<void> => {
    let upsertBranchPropertiesRes = await browser.executeAsync(
        modelNodeUpsertPropertiesScript,
        modelItemId,
        properties
    );

    if (typeof upsertBranchPropertiesRes === "string") {
        assert.fail(upsertBranchPropertiesRes);
    }

    console.log("Upserted viewport item:");
    console.log(JSON.stringify(upsertBranchPropertiesRes, null, 2));

    for (let vpId of Object.keys(upsertBranchPropertiesRes)) {
        let vpVpItems = upsertBranchPropertiesRes[vpId];
        vpVpItemsLoop: for (let vpItem of vpVpItems) {
            for (let prop_key in properties) {
                if (vpItem.modelData[prop_key] != properties[prop_key]) {
                    continue vpVpItemsLoop;
                }
            }
            return;
        }
    }

    assert.fail("Node properties were not set!");
};
