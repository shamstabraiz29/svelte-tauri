import { browser } from "@wdio/globals";
import assert from "assert";

import { saveUpdatesScript } from "./scripts/index.ts";
import { TrackedViewportItem, ViewportItemMeta } from "tauri-plugin-editor";

export default async (
    viewportItems: TrackedViewportItem[],
    nonUpsertedVpItems: string[]
): Promise<void> => {
    let saveUpdatedRes = await browser.executeAsync(
        saveUpdatesScript,
        viewportItems
    );

    if (typeof saveUpdatedRes === "string") {
        assert.fail(saveUpdatedRes);
    }

    let allUpsertedViewportItems: { [x: string]: ViewportItemMeta } = {};

    for (let viewportItems of Object.values(saveUpdatedRes)) {
        for (let viewportItem of viewportItems) {
            allUpsertedViewportItems[viewportItem.id] = viewportItem;
        }
    }

    for (let vpItem of viewportItems) {
        let vpItemRes = allUpsertedViewportItems[vpItem.vpId];

        if (nonUpsertedVpItems.includes(vpItem.vpId)) {
            if (vpItemRes) {
                assert.fail("Viewport item was upserted!");
            }
            continue;
        }

        if (!vpItemRes) {
            assert.fail("Viewport item was not upserted!");
        }

        for (let prop_key in vpItem.vpDelta.upsertedProperties) {
            if (!vpItem.vpDelta.removedProperties.includes(prop_key)) {
                assert.deepEqual(
                    vpItemRes.properties[prop_key],
                    vpItem.vpDelta.upsertedProperties[prop_key]
                );
            }
        }

        for (let prop_key in vpItem.vpDelta.removedProperties) {
            assert.strictEqual(vpItemRes.properties[prop_key], undefined);
        }

        for (let prop_key in vpItem.mDelta.upsertedProperties) {
            if (!vpItem.mDelta.removedProperties.includes(prop_key)) {
                assert.deepEqual(
                    vpItemRes.modelData[prop_key],
                    vpItem.mDelta.upsertedProperties[prop_key]
                );
            }
        }

        for (let prop_key in vpItem.mDelta.removedProperties) {
            assert.strictEqual(vpItemRes.modelData[prop_key], undefined);
        }
    }
};
