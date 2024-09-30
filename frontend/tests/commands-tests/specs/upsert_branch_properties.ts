import { browser } from "@wdio/globals";
import assert from "assert";

import { upsertBranchPropertiesScript } from "./scripts/index.ts";
import { RepoDetail } from "tauri-plugin-repo-client";

export default async (
    branchId: string,
    properties: any
): Promise<RepoDetail> => {
    let upsertBranchPropertiesRes = await browser.executeAsync(
        upsertBranchPropertiesScript,
        branchId,
        properties
    );

    if (typeof upsertBranchPropertiesRes === "string") {
        assert.fail(upsertBranchPropertiesRes);
    }

    let targetBranch = upsertBranchPropertiesRes.branches[branchId];

    if (!targetBranch) {
        assert.fail("No branch with the provided ID was found in the repo!");
    }

    assert.deepEqual(targetBranch.properties, properties);

    return upsertBranchPropertiesRes;
};
