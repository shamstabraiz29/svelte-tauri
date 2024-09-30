import { browser } from "@wdio/globals";
import assert from "assert";

import { evaluateCloudPatternScript } from "./scripts/index.ts";
import {
    CloudPatternRequestResponse,
    DropInfoRequest,
    EvaluateCloudPatternRequest,
    PropertiesValuesRequest,
} from "tauri-plugin-editor";

export default async (
    dropInfoRequest: DropInfoRequest,
    testViewportId: string
): Promise<PropertiesValuesRequest> => {
    let validDropTargetNode =
        dropInfoRequest?.validDropLocations[0]?.viewportItemId;

    if (!validDropTargetNode) {
        assert.fail("No valid drop target was found!");
    }

    let respondingWith: CloudPatternRequestResponse = {
        type: "dropLocationInfo",
        inContextName: dropInfoRequest.inContextName,
        x: 0,
        y: 0,
        viewportId: testViewportId,
        droppedOnNode: validDropTargetNode,
    };

    let payload: EvaluateCloudPatternRequest = {
        reqId: "drop-information",
        respondingWith,
    };

    let evaluateCloudPatternRes = await browser.executeAsync(
        evaluateCloudPatternScript,
        payload,
        false
    );

    // console.log(evaluateCloudPatternRes);

    if (typeof evaluateCloudPatternRes === "string") {
        assert.fail(evaluateCloudPatternRes);
    }

    let propertiesValuesRequest =
        evaluateCloudPatternRes as PropertiesValuesRequest;
    if (!propertiesValuesRequest.requests) {
        assert.fail("Expected a PropertiesValuesRequest!");
    }

    // console.log(JSON.stringify(propertiesValuesRequest, null, 4));

    return propertiesValuesRequest;
};
