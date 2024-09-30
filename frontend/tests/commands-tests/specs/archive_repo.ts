import { browser } from "@wdio/globals";
import assert from "assert";

import { repoArchiveScript } from "./scripts/index.ts";
import { AccountDetail } from "tauri-plugin-account-client";
import { findRepo } from "./helpers/index.ts";

export default async (
    repoId: string,
    parentFolders: string[]
): Promise<AccountDetail> => {
    let repoRemoveRes = await browser.executeAsync(repoArchiveScript, repoId);

    if (typeof repoRemoveRes === "string") {
        assert.fail(repoRemoveRes);
    }

    let targetRepo = findRepo(repoId, parentFolders, repoRemoveRes);

    assert.strictEqual(targetRepo, null);

    return repoRemoveRes;
};
