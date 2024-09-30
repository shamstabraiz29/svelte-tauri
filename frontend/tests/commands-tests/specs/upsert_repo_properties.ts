import { browser } from "@wdio/globals";
import assert from "assert";

import { upsertRepoPropertiesScript } from "./scripts/index.ts";
import { AccountDetail } from "tauri-plugin-account-client";
import { findRepo } from "./helpers/index.ts";

export default async (
    repoId: string,
    properties: any,
    parentRepos: string[]
): Promise<AccountDetail> => {
    let upsertRepoPropertiesRes = await browser.executeAsync(
        upsertRepoPropertiesScript,
        repoId,
        properties
    );

    if (typeof upsertRepoPropertiesRes === "string") {
        assert.fail(upsertRepoPropertiesRes);
    }

    let targetRepo = findRepo(repoId, parentRepos, upsertRepoPropertiesRes);

    if (!targetRepo) {
        assert.fail("No repo with the provided ID was found in the account!");
    }

    assert.deepEqual(targetRepo.properties, properties);

    return upsertRepoPropertiesRes;
};
