import { browser } from "@wdio/globals";
import assert from "assert";

import { evaluateCloudPatternScript } from "./scripts/index.ts";
import {
    CloudPatternRequestResponse,
    DropInfoRequest,
    EvaluateCloudPatternRequest,
    PropertyValueRequestResponse,
} from "tauri-plugin-editor";

export default async (
    propertiesValues: PropertyValueRequestResponse[],
    composed: boolean
): Promise<DropInfoRequest | null> => {
    let respondingWith: CloudPatternRequestResponse = {
        type: "propertiesValues",
        responses: propertiesValues,
    };

    let payload: EvaluateCloudPatternRequest = {
        reqId: "providing-properties-values",
        respondingWith,
    };

    let evaluateCloudPatternRes = await browser.executeAsync(
        evaluateCloudPatternScript,
        payload,
        composed
    );

    if (typeof evaluateCloudPatternRes === "string") {
        assert.fail(evaluateCloudPatternRes);
    }

    let dropInfoRequest: DropInfoRequest =
        evaluateCloudPatternRes as DropInfoRequest;
    if (dropInfoRequest.cursorIconUrl) {
        return dropInfoRequest;
    }

    return null;
};
