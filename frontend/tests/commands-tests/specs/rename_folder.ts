import { browser } from "@wdio/globals";
import assert from "assert";

import { folderRenameScript } from "./scripts/index.ts";
import { AccountDetail } from "tauri-plugin-account-client";
import { findFolder } from "./helpers/index.ts";

export default async (
    folderId: string,
    newFolderName: string,
    parentFolders: string[]
): Promise<AccountDetail> => {
    let folderRenameRes = await browser.executeAsync(
        folderRenameScript,
        folderId,
        newFolderName
    );

    if (typeof folderRenameRes === "string") {
        assert.fail(folderRenameRes);
    }

    let targetFolder = findFolder(
        folderId,
        parentFolders,
        folderRenameRes.rootFolders
    );

    if (!targetFolder) {
        assert.fail("No folder with the provided ID was found in the account!");
    }

    assert.strictEqual(targetFolder.name, newFolderName);

    return folderRenameRes;
};
