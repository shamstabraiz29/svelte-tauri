import { browser } from "@wdio/globals";
import assert from "assert";

import { AccountDetail } from "tauri-plugin-account-client";
import { accountSetScript } from "./scripts/index.ts";

export default async (accountId: string): Promise<AccountDetail> => {
    const accountSetResponse = await browser.executeAsync(
        accountSetScript,
        accountId
    );

    if (typeof accountSetResponse === "string") {
        assert.fail(accountSetResponse);
    }

    console.log("Setting account to: ");
    console.log(JSON.stringify(accountSetResponse, null, 2));

    return accountSetResponse;
};
