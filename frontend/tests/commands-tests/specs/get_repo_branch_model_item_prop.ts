import assert from "assert";

import { AccountDetail } from "tauri-plugin-account-client";
import setOpenRepo from "./set_open_repo.ts";
import loadModel from "./load_model.ts";
import getResourceItemSchema from "./get_resource_item_schema.ts";

export default async (
    accountSetRes: AccountDetail
): Promise<{
    repoId: string;
    branchId: string;
    vpItemId: string;
    vpItemModelItemId: string;
    modelItemPropName: string;
}> => {
    let repoId: string | null = null;
    let branchId: string | null = null;
    let vpItemId: string | null = null;
    let vpItemModelItemId: string | null = null;
    let modelItemPropName: string | null = null;

    let repoIds: string[] = [];

    for (let repo of accountSetRes.rootRepos) {
        repoIds.push(repo.id);
    }

    for (let folder of accountSetRes.rootFolders) {
        for (let repo of folder.repoMetas) {
            repoIds.push(repo.id);
        }
    }

    console.log("Iterating over repos: ");
    console.log(JSON.stringify(repoIds, null, 2));

    rootLoop: for (let repoIdInner of repoIds) {
        const getRepoRes = await setOpenRepo(repoIdInner);

        for (let branchDetails of Object.values(getRepoRes.branches)) {
            const loadModelRes = await loadModel(branchDetails.id);
            if (typeof loadModelRes == "string") {
                assert.fail(loadModelRes);
            }
            console.log(JSON.stringify(loadModelRes, null, 2));
            for (let vpId in loadModelRes.viewportItems) {
                console.log(
                    `Iterating over viewport items for viewport: ${vpId}`
                );
                let vpVpItems = loadModelRes.viewportItems[vpId];
                for (let vpItem of vpVpItems) {
                    let vpItemNodeTypeInfo = await getResourceItemSchema(
                        vpItem.resourceItemType
                    );
                    if (vpItemNodeTypeInfo.type === "Relationship") {
                        continue;
                    }
                    console.log(
                        "Looking for string property in model item: ",
                        vpItem.id
                    );
                    for (let propName of Object.keys(
                        vpItemNodeTypeInfo.properties
                    )) {
                        if (
                            vpItemNodeTypeInfo.properties[propName].valueType
                                .type === "Text"
                        ) {
                            console.log("Found text property: ", propName);
                            repoId = repoIdInner;
                            branchId = branchDetails.id;
                            vpItemId = vpItem.id;
                            vpItemModelItemId = vpItem.resourceItemId;
                            modelItemPropName = propName;
                            break rootLoop;
                        }
                    }
                    console.log(
                        "No string property found in model item: ",
                        vpItem.id
                    );
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

    if (!vpItemId) {
        assert.fail("No viewport item was found in any model!");
    }

    if (!vpItemModelItemId) {
        assert.fail("No model item was found in any model!");
    }

    if (!modelItemPropName) {
        assert.fail("No model item property was found in any model item!");
    }

    return {
        repoId,
        branchId,
        vpItemId,
        vpItemModelItemId,
        modelItemPropName,
    };
};
