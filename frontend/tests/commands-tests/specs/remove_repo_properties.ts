import { browser } from "@wdio/globals";
import assert from "assert";

import { removeRepoPropertiesScript } from "./scripts/index.ts";
import { AccountDetail } from "tauri-plugin-account-client";
import { findRepo } from "./helpers/index.ts";

export default async (
    repoId: string,
    properties: string[],
    parentFolders: string[],
    expectedProperties: any
): Promise<AccountDetail> => {
    let removeRepoPropertiesRes = await browser.executeAsync(
        removeRepoPropertiesScript,
        repoId,
        properties
    );

    if (typeof removeRepoPropertiesRes === "string") {
        assert.fail(removeRepoPropertiesRes);
    }

    let targetRepo = findRepo(repoId, parentFolders, removeRepoPropertiesRes);

    if (!targetRepo) {
        assert.fail("No repo with the provided ID was found in the account!");
    }

    assert.deepEqual(targetRepo.properties, expectedProperties);

    return removeRepoPropertiesRes;
};
