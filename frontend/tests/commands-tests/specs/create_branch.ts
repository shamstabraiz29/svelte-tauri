import { browser } from "@wdio/globals";
import assert from "assert";

import { branchCreateScript } from "./scripts/index.ts";
import {
    RepoDetail,
    ParentBranchPointDetail,
    BranchMetaDetail,
} from "tauri-plugin-repo-client";

export type BranchCreated = {
    branchId: string;
    repoDetail: RepoDetail;
};

export default async (
    name: string,
    parentBranch: ParentBranchPointDetail | null,
    properties: { [x: string]: unknown },
    siblingBranches: BranchMetaDetail[]
): Promise<BranchCreated> => {
    let branchCreateRes = await browser.executeAsync(
        branchCreateScript,
        name,
        parentBranch,
        properties
    );

    let testBranchId: string | null = null;

    if (typeof branchCreateRes === "string") {
        assert.fail(branchCreateRes);
    }

    let siblingBranchIds = siblingBranches.map((branch) => branch.id);

    for (let branch of Object.values(branchCreateRes.branches)) {
        if (siblingBranchIds.includes(branch.id)) {
            continue;
        }
        testBranchId = branch.id;
        break;
    }

    if (!testBranchId) {
        assert.fail("New branch was not found!");
    }

    return { branchId: testBranchId, repoDetail: branchCreateRes };
};
