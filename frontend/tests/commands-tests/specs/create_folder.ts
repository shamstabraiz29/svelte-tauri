import { browser } from "@wdio/globals";
import assert from "assert";

import { folderCreateScript } from "./scripts/index.ts";
import { AccountDetail, Folder } from "tauri-plugin-account-client";

export type FolderCreated = {
    folderId: string;
    acctDetail: AccountDetail;
};

export default async (
    rootFolderId: string,
    siblingFolders: Folder[]
): Promise<FolderCreated> => {
    let folderCreateRes = await browser.executeAsync(
        folderCreateScript,
        rootFolderId
    );

    let testFolderId: string | null = null;

    if (typeof folderCreateRes === "string") {
        assert.fail(folderCreateRes);
    }

    let siblingFolderIds = siblingFolders.map((folder) => folder.id);

    for (let folder of Object.values(folderCreateRes.rootFolders)) {
        if (siblingFolderIds.includes(folder.id)) {
            continue;
        }
        testFolderId = folder.id;
        break;
    }

    if (!testFolderId) {
        assert.fail("No viewport ID found!");
    }

    return { folderId: testFolderId, acctDetail: folderCreateRes };
};
