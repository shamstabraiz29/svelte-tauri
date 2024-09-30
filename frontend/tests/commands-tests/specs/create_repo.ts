import { browser } from "@wdio/globals";
import assert from "assert";

import { repoCreateScript } from "./scripts/index.ts";
import { AccountDetail, RepoMeta } from "tauri-plugin-account-client";

export type RepoCreated = {
    repoId: string;
    acctDetail: AccountDetail;
};

export default async (
    rootRepoId: string,
    siblingRepos: RepoMeta[]
): Promise<RepoCreated> => {
    let repoCreateRes = await browser.executeAsync(
        repoCreateScript,
        rootRepoId
    );

    let testRepoId: string | null = null;

    if (typeof repoCreateRes === "string") {
        assert.fail(repoCreateRes);
    }

    let siblingRepoIds = siblingRepos.map((repo) => repo.id);

    for (let repo of Object.values(repoCreateRes.rootRepos)) {
        if (siblingRepoIds.includes(repo.id)) {
            continue;
        }
        testRepoId = repo.id;
        break;
    }

    if (!testRepoId) {
        assert.fail("No Repo was found!");
    }

    return { repoId: testRepoId, acctDetail: repoCreateRes };
};
