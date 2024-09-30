import { browser } from "@wdio/globals";
import assert from "assert";

import { upsertFolderPropertiesScript } from "./scripts/index.ts";
import { AccountDetail } from "tauri-plugin-account-client";
import { findFolder } from "./helpers/index.ts";

export default async (
    folderId: string,
    properties: any,
    parentFolders: string[]
): Promise<AccountDetail> => {
    let upsertFolderPropertiesRes = await browser.executeAsync(
        upsertFolderPropertiesScript,
        folderId,
        properties
    );

    if (typeof upsertFolderPropertiesRes === "string") {
        assert.fail(upsertFolderPropertiesRes);
    }

    let targetFolder = findFolder(
        folderId,
        parentFolders,
        upsertFolderPropertiesRes.rootFolders
    );

    if (!targetFolder) {
        assert.fail("No folder with the provided ID was found in the account!");
    }

    assert.deepEqual(targetFolder.properties, properties);

    return upsertFolderPropertiesRes;
};
