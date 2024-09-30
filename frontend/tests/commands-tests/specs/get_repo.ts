import { browser } from "@wdio/globals";
import assert from "assert";

import { AccountDetail } from "tauri-plugin-account-client";
import { Result } from "./result.ts";
import { RepoResponse } from "tauri-plugin-repo-client";
import { getRepoScript } from "./scripts/index.ts";
import { Error as LoginError } from "tauri-plugin-cognito-login";

export default async (
    accountSetRes: AccountDetail
): Promise<{
    testRepo1Id: string;
    testBranch1Id: string;
    testRepo2Id: string;
    testBranch2Id: string;
}> => {
    let testRepo1Id: string | null = null;
    let testBranch1Id: string | null = null;
    let testRepo2Id: string | null = null;
    let testBranch2Id: string | null = null;

    let repoIds: string[] = [];

    for (let repo of accountSetRes.rootRepos) {
        repoIds.push(repo.id);
    }

    for (let folder of accountSetRes.rootFolders) {
        for (let repo of folder.repoMetas) {
            repoIds.push(repo.id);
        }
    }

    let testRepoIdInner: string | null = null;
    let testBranchIdInner: string | null = null;
    let count = 0;

    for (let repoId of repoIds) {
        const getRepoRes: Result<RepoResponse, LoginError> =
            await browser.executeAsync(getRepoScript, repoId);

        if (getRepoRes.error) {
            assert.fail(getRepoRes.error.message);
        }

        if (!getRepoRes.data) {
            assert.fail("No data returned");
        }

        let repoBranches = getRepoRes.data.repoDetail;

        for (let branchDetails of Object.values(repoBranches.branches)) {
            testBranchIdInner = branchDetails.id;
            testRepoIdInner = repoBranches.id;
            count++;
            break;
        }

        if (testRepoIdInner && testBranchIdInner) {
            if (count > 1) {
                testRepo2Id = testRepoIdInner;
                testBranch2Id = testBranchIdInner;
                break;
            } else {
                testRepo1Id = testRepoIdInner;
                testBranch1Id = testBranchIdInner;
            }
        }
    }

    if (!testRepo1Id) {
        assert.fail("No repo with a branch was found!");
    }

    if (!testBranch1Id) {
        assert.fail("No branch was found!");
    }

    if (!testRepo2Id) {
        assert.fail("Second repo with a branch not found!");
    }

    if (!testBranch2Id) {
        assert.fail("Second test branch not found!");
    }

    return {
        testRepo1Id,
        testBranch1Id,
        testRepo2Id,
        testBranch2Id,
    };
};
