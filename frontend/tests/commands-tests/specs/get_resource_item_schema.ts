import { browser } from "@wdio/globals";
import assert from "assert";

import { ResourceItemSchemaDto } from "tauri-plugin-editor";
import { getResourceItemSchemaScript } from "./scripts/index.ts";

export default async (
    resourceItemType: string
): Promise<ResourceItemSchemaDto> => {
    let getResourceItemSchemaRes = await browser.executeAsync(
        getResourceItemSchemaScript,
        resourceItemType
    );

    if (typeof getResourceItemSchemaRes === "string") {
        assert.fail(getResourceItemSchemaRes);
    }

    return getResourceItemSchemaRes;
};
