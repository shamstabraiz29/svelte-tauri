import { browser } from "@wdio/globals";
import assert from "assert";

import { repoRenameScript } from "./scripts/index.ts";
import { AccountDetail } from "tauri-plugin-account-client";
import { findRepo } from "./helpers/index.ts";

export default async (
    repoId: string,
    newRepoName: string,
    parentFolders: string[]
): Promise<AccountDetail> => {
    let repoRenameRes = await browser.executeAsync(
        repoRenameScript,
        repoId,
        newRepoName
    );

    if (typeof repoRenameRes === "string") {
        assert.fail(repoRenameRes);
    }

    let targetRepo = findRepo(repoId, parentFolders, repoRenameRes);

    if (!targetRepo) {
        assert.fail("No repo with the provided ID was found in the account!");
    }

    assert.strictEqual(targetRepo.name, newRepoName);

    return repoRenameRes;
};
