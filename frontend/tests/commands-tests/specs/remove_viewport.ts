import { browser } from "@wdio/globals";
import assert from "assert";

import { viewportRemoveScript } from "./scripts/index.ts";

export default async (viewportId: string): Promise<void> => {
    let removeViewportRes = await browser.executeAsync(
        viewportRemoveScript,
        viewportId
    );

    if (typeof removeViewportRes === "string") {
        assert.fail(removeViewportRes);
    }

    let targetViewportId = removeViewportRes[0] || null;

    if (!targetViewportId) {
        assert.fail("Received an unexpected response!");
    }

    assert.equal(targetViewportId, viewportId);
};
