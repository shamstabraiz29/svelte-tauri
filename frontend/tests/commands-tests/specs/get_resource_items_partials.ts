import { browser } from "@wdio/globals";
import assert from "assert";

import { getResourceItemsPartialsScript } from "./scripts/index.ts";

export default async (
    viewportType: string,
    resourceItemsTypes: string[]
): Promise<unknown> => {
    let getResourceItemsPartialsRes = await browser.executeAsync(
        getResourceItemsPartialsScript,
        viewportType,
        resourceItemsTypes
    );

    if (typeof getResourceItemsPartialsRes === "string") {
        assert.fail(getResourceItemsPartialsRes);
    }

    return getResourceItemsPartialsRes;
};
