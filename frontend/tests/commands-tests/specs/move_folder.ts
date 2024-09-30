import { browser } from "@wdio/globals";
import assert from "assert";

import { folderMoveScript } from "./scripts/index.ts";
import { AccountDetail } from "tauri-plugin-account-client";
import { findFolder } from "./helpers/index.ts";

export default async (
    folderToMoveId: string,
    parentFolderId: string,
    parentFolders: string[]
): Promise<AccountDetail> => {
    let folderMoveRes = await browser.executeAsync(
        folderMoveScript,
        folderToMoveId,
        parentFolderId
    );

    if (typeof folderMoveRes === "string") {
        assert.fail(folderMoveRes);
    }

    let targetFolder = findFolder(
        folderToMoveId,
        parentFolders,
        folderMoveRes.rootFolders
    );

    if (!targetFolder) {
        assert.fail("No folder with the provided ID was found in the account!");
    }

    return folderMoveRes;
};
