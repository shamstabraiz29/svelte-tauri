import { browser } from "@wdio/globals";
import assert from "assert";

import { Credentials } from "./load_token.ts";
import {
    Error as LoginError,
    PwLoginResponse,
} from "tauri-plugin-cognito-login";
import { pwLoginScript } from "./scripts/index.ts";
import { Result } from "./result.ts";

declare var it: any;

declare global {
    namespace WebdriverIO {
        interface Browser {
            loadCredentials: () => Promise<Credentials>;
        }
    }
}

export default async (): Promise<{ session: string; username: string }> => {
    // Remove this if you need a headache
    await browser.pause(1000);

    await browser.setTimeout({ script: 120000 });

    let credentials = await browser.loadCredentials();

    let username = credentials.username;

    console.log("*".repeat(80));
    console.log(credentials.username);
    console.log(credentials.password);
    console.log("*".repeat(80));

    const pwLoginRes: Result<PwLoginResponse, LoginError> =
        await browser.executeAsync(
            pwLoginScript,
            credentials.username,
            credentials.password
        );

    if (pwLoginRes.error) {
        assert.fail(pwLoginRes.error.message);
    }

    if (!pwLoginRes.data) {
        assert.fail("No data returned");
    }

    assert.strictEqual(pwLoginRes.data.state.type, "mfaRequired");

    let session = pwLoginRes.data.state.session;

    return { session, username };
};
