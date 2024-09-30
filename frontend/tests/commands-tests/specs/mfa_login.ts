import {
    Error as LoginError,
    MfaLoginResponse,
} from "tauri-plugin-cognito-login";

import { browser, $ } from "@wdio/globals";
import assert from "assert";

import { mfaLoginScript } from "./scripts/index.ts";

import { Result } from "./result.ts";

export default async (username: string, session: string) => {
    const mfaCodeSetSpan = await $("#mfaCodeSet");
    await mfaCodeSetSpan.waitForExist({ timeout: 60000 });

    let mfaCode = await (await $("#mfaCode")).getValue();

    const mfaLoginRes: Result<MfaLoginResponse, LoginError> =
        await browser.executeAsync(mfaLoginScript, username, mfaCode, session);

    if (mfaLoginRes.error) {
        assert.fail(mfaLoginRes.error.message);
    }

    if (!mfaLoginRes.data) {
        assert.fail("No data returned");
    }
};
