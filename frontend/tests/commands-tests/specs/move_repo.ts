import { browser } from "@wdio/globals";
import assert from "assert";

import { repoMoveScript } from "./scripts/index.ts";
import { AccountDetail } from "tauri-plugin-account-client";
import { findRepo } from "./helpers/index.ts";

export default async (
    repoToMoveId: string,
    parentFolderId: string,
    parentFolders: string[]
): Promise<AccountDetail> => {
    let repoMoveRes = await browser.executeAsync(
        repoMoveScript,
        repoToMoveId,
        parentFolderId
    );

    if (typeof repoMoveRes === "string") {
        assert.fail(repoMoveRes);
    }

    let targetRepo = findRepo(repoToMoveId, parentFolders, repoMoveRes);

    if (!targetRepo) {
        assert.fail("No repo with the provided ID was found in the account!");
    }

    return repoMoveRes;
};
