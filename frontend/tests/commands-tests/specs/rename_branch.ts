import { browser } from "@wdio/globals";
import assert from "assert";

import { branchRenameScript } from "./scripts/index.ts";
import { RepoDetail } from "tauri-plugin-repo-client";

export default async (
    branchId: string,
    newBranchName: string
): Promise<RepoDetail> => {
    let branchRenameRes = await browser.executeAsync(
        branchRenameScript,
        branchId,
        newBranchName
    );

    if (typeof branchRenameRes === "string") {
        assert.fail(branchRenameRes);
    }

    let targetBranch = branchRenameRes.branches[branchId];

    if (!targetBranch) {
        assert.fail("No branch with the provided ID was found in the repo!");
    }

    assert.strictEqual(targetBranch.name, newBranchName);

    return branchRenameRes;
};
