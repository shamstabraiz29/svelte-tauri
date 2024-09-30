import { browser } from "@wdio/globals";
import assert from "assert";

import { getResourceItemPartialsScript } from "./scripts/index.ts";

export default async (
    viewportType: string,
    resourceItemType: string
): Promise<unknown> => {
    let getResourceItemPartialsRes = await browser.executeAsync(
        getResourceItemPartialsScript,
        viewportType,
        resourceItemType
    );

    if (typeof getResourceItemPartialsRes === "string") {
        assert.fail(getResourceItemPartialsRes);
    }

    return getResourceItemPartialsRes;
};
