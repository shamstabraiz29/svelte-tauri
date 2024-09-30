import { browser } from "@wdio/globals";
import assert from "assert";

import { RepoDetail } from "tauri-plugin-repo-client";
import { setRepoScript } from "./scripts/index.ts";

export default async (testRepo1Id: string): Promise<RepoDetail> => {
    const setRepoRes = await browser.executeAsync(setRepoScript, testRepo1Id);

    if (typeof setRepoRes === "string") {
        assert.fail(setRepoRes);
    }

    console.log("Set repo to: ");
    console.log(JSON.stringify(setRepoRes, null, 2));

    return setRepoRes;
};
