import { browser } from "@wdio/globals";
import assert from "assert";

import { viewportCreateScript } from "./scripts/index.ts";

export default async (): Promise<string> => {
    let viewportCreateRes = await browser.executeAsync(
        viewportCreateScript,
        "Infra Viewport 1",
        "infrastructure",
        {}
    );

    let testViewportId: string | null = null;

    if (typeof viewportCreateRes === "string") {
        assert.fail(viewportCreateRes);
    }

    for (let viewport of Object.values(viewportCreateRes)) {
        testViewportId = viewport.id;
        break;
    }

    if (!testViewportId) {
        assert.fail("No viewport ID found!");
    }

    // console.log(JSON.stringify(viewportCreateRes));

    return testViewportId;
};
