import { browser } from "@wdio/globals";
import assert from "assert";

import { folderRemoveScript } from "./scripts/index.ts";
import { AccountDetail } from "tauri-plugin-account-client";
import { findFolder } from "./helpers/index.ts";

export default async (
    folderId: string,
    parentFolders: string[]
): Promise<AccountDetail> => {
    let folderRemoveRes = await browser.executeAsync(
        folderRemoveScript,
        folderId
    );

    if (typeof folderRemoveRes === "string") {
        assert.fail(folderRemoveRes);
    }

    let targetFolder = findFolder(
        folderId,
        parentFolders,
        folderRemoveRes.rootFolders
    );

    assert.strictEqual(targetFolder, null);

    return folderRemoveRes;
};
