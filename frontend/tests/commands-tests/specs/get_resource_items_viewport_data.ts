import { browser } from "@wdio/globals";
import assert from "assert";

import { getResourceItemsViewportDataScript } from "./scripts/index.ts";

export default async (
    viewportType: string,
    resourceItemsTypes: string[]
): Promise<unknown> => {
    let getResourceItemsViewportDataRes = await browser.executeAsync(
        getResourceItemsViewportDataScript,
        viewportType,
        resourceItemsTypes
    );

    if (typeof getResourceItemsViewportDataRes === "string") {
        assert.fail(getResourceItemsViewportDataRes);
    }

    return getResourceItemsViewportDataRes;
};
