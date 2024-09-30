import { browser } from "@wdio/globals";
import assert from "assert";

import { loadModelScript } from "./scripts/index.ts";
import { BranchDetail } from "tauri-plugin-editor";

export default async (branchId: string): Promise<BranchDetail> => {
    const loadModelRes = await browser.executeAsync(loadModelScript, branchId);

    if (typeof loadModelRes == "string") {
        assert.fail(loadModelRes);
    }

    console.log("Set loaded model to: ");
    console.log(JSON.stringify(loadModelRes, null, 2));

    return loadModelRes;
};
