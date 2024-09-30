import { browser } from "@wdio/globals";
import assert from "assert";

import { removeBranchPropertiesScript } from "./scripts/index.ts";
import { RepoDetail } from "tauri-plugin-repo-client";

export default async (
    branchId: string,
    properties: string[],
    expectedProperties: any
): Promise<RepoDetail> => {
    let removeBranchPropertiesRes = await browser.executeAsync(
        removeBranchPropertiesScript,
        branchId,
        properties
    );

    if (typeof removeBranchPropertiesRes === "string") {
        assert.fail(removeBranchPropertiesRes);
    }

    let targetBranch = removeBranchPropertiesRes.branches[branchId];

    if (!targetBranch) {
        assert.fail("No branch with the provided ID was found in the repo!");
    }

    assert.deepEqual(targetBranch.properties, expectedProperties);

    return removeBranchPropertiesRes;
};
