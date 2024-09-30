import { browser } from "@wdio/globals";
import assert from "assert";

import { getCloudPatternsScript } from "./scripts/index.ts";

export default async () => {
    let getCloudPatternsRes = await browser.executeAsync(
        getCloudPatternsScript
    );

    if (typeof getCloudPatternsRes === "string") {
        assert.fail(getCloudPatternsRes);
    }

    // console.log(getCloudPatternsRes);
};
