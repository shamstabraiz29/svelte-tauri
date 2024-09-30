import { browser } from "@wdio/globals";
import assert from "assert";

import { AccountDetail } from "tauri-plugin-account-client";
import { loadModelScript } from "./scripts/index.ts";
import setOpenRepo from "./set_open_repo.ts";

export default async (
    accountSetRes: AccountDetail
): Promise<{
    repoId: string;
    branchId: string;
    viewportId: string;
    viewportItemId: string;
}> => {
    let repoId: string | null = null;
    let branchId: string | null = null;
    let viewportId: string | null = null;
    let viewportItemId: string | null = null;

    let repoIds: string[] = [];

    for (let repo of accountSetRes.rootRepos) {
        repoIds.push(repo.id);
    }

    for (let folder of accountSetRes.rootFolders) {
        for (let repo of folder.repoMetas) {
            repoIds.push(repo.id);
        }
    }

    rootLoop: for (let repoIdInner of repoIds) {
        const getRepoRes = await setOpenRepo(repoIdInner);

        for (let branchDetails of Object.values(getRepoRes.branches)) {
            const loadModelRes = await browser.executeAsync(
                loadModelScript,
                branchDetails.id
            );
            if (typeof loadModelRes == "string") {
                assert.fail(loadModelRes);
            }
            console.log(JSON.stringify(loadModelRes, null, 2));
            let canvasesIds: string[] = [];
            for (let viewport of Object.values(loadModelRes.viewports)) {
                canvasesIds.push(viewport.properties["canvasId"] as string);
            }
            for (let vpId in loadModelRes.viewportItems) {
                let vpVpItems = loadModelRes.viewportItems[vpId];
                for (let vpItem of vpVpItems) {
                    if (!canvasesIds.includes(vpItem.id)) {
                        console.log("Found viewport item in model!");
                        console.log("Repo ID: " + repoIdInner);
                        console.log("Branch ID: " + branchDetails.id);
                        console.log("Viewport ID: " + vpId);
                        console.log("Viewport Item ID: " + vpItem.id);
                        repoId = repoIdInner;
                        branchId = branchDetails.id;
                        viewportId = vpId;
                        viewportItemId = vpItem.id;
                        break rootLoop;
                    }
                }
            }
        }
    }

    if (!repoId) {
        assert.fail("No repo with a branch was found!");
    }

    if (!branchId) {
        assert.fail("No branch was found!");
    }

    if (!viewportId) {
        assert.fail("No viewport was found in any model!");
    }

    if (!viewportItemId) {
        assert.fail("No viewport item was found in any model!");
    }

    return {
        repoId,
        branchId,
        viewportId,
        viewportItemId,
    };
};
