import { browser } from "@wdio/globals";
import assert from "assert";

import { getResourceItemViewportDataScript } from "./scripts/index.ts";

export default async (
    viewportType: string,
    resourceItemType: string
): Promise<unknown> => {
    let getResourceItemViewportDataRes = await browser.executeAsync(
        getResourceItemViewportDataScript,
        viewportType,
        resourceItemType
    );

    if (typeof getResourceItemViewportDataRes === "string") {
        assert.fail(getResourceItemViewportDataRes);
    }

    return getResourceItemViewportDataRes;
};
