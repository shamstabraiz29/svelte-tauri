import { browser } from "@wdio/globals";
import assert from "assert";

import { branchRemoveScript } from "./scripts/index.ts";
import { RepoDetail } from "tauri-plugin-repo-client";

export default async (branchId: string): Promise<RepoDetail> => {
    let branchRemoveRes = await browser.executeAsync(
        branchRemoveScript,
        branchId
    );

    if (typeof branchRemoveRes === "string") {
        assert.fail(branchRemoveRes);
    }

    let targetBranch = branchRemoveRes.branches[branchId] || null;

    assert.strictEqual(targetBranch, null);

    return branchRemoveRes;
};
