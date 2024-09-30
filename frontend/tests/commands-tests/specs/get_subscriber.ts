import { SubscriberResponse } from "tauri-plugin-subscriber-client";
import { Error as LoginError } from "tauri-plugin-cognito-login";

import { browser } from "@wdio/globals";
import assert from "assert";

import { getSubscriberScript } from "./scripts/index.ts";

import { Result } from "./result.ts";

export default async (): Promise<{ username: string; accountId: string }> => {
    const getSubscriberRes: Result<SubscriberResponse, LoginError> =
        await browser.executeAsync(getSubscriberScript);

    if (getSubscriberRes.error) {
        assert.fail(getSubscriberRes.error.message);
    }

    if (!getSubscriberRes.data) {
        assert.fail("No data returned");
    }

    if (getSubscriberRes.data.subscriberDetail.acctIds[0]) {
        let username = getSubscriberRes.data.subscriberDetail.id;
        let accountId = getSubscriberRes.data.subscriberDetail.acctIds[0];
        return { username, accountId };
    } else {
        assert.fail("No account ID found");
    }
};
