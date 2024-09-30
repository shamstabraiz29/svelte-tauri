import { browser } from "@wdio/globals";
import assert from "assert";

import { evaluateCloudPatternScript } from "./scripts/index.ts";
import {
    DropInfoRequest,
    EvaluateCloudPatternRequest,
} from "tauri-plugin-editor";

export default async (cloudPatternId: string): Promise<DropInfoRequest> => {
    let payload: EvaluateCloudPatternRequest = {
        reqId: "double-click-cloud-pattern",
        respondingWith: { type: "init", cloudPatternId },
    };

    let evaluateCloudPatternRes = await browser.executeAsync(
        evaluateCloudPatternScript,
        payload,
        false
    );

    if (typeof evaluateCloudPatternRes === "string") {
        assert.fail(evaluateCloudPatternRes);
    }

    let dropInfoRequest: DropInfoRequest =
        evaluateCloudPatternRes as DropInfoRequest;
    if (!dropInfoRequest.cursorIconUrl) {
        assert.fail("Expected a DropInfoRequest!");
    }

    // console.log("Drop info request: ", dropInfoRequest);

    return dropInfoRequest;
};
