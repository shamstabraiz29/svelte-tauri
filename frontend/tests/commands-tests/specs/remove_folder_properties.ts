import { browser } from "@wdio/globals";
import assert from "assert";

import { removeFolderPropertiesScript } from "./scripts/index.ts";
import { AccountDetail } from "tauri-plugin-account-client";
import { findFolder } from "./helpers/index.ts";

export default async (
    folderId: string,
    properties: string[],
    parentFolders: string[],
    expectedProperties: any
): Promise<AccountDetail> => {
    let removeFolderPropertiesRes = await browser.executeAsync(
        removeFolderPropertiesScript,
        folderId,
        properties
    );

    if (typeof removeFolderPropertiesRes === "string") {
        assert.fail(removeFolderPropertiesRes);
    }

    let targetFolder = findFolder(
        folderId,
        parentFolders,
        removeFolderPropertiesRes.rootFolders
    );

    if (!targetFolder) {
        assert.fail("No folder with the provided ID was found in the account!");
    }

    assert.deepEqual(targetFolder.properties, expectedProperties);

    return removeFolderPropertiesRes;
};
