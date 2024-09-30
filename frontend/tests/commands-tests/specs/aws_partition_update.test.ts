import { AccountDetail } from "tauri-plugin-account-client";

import getSubscriber from "./get_subscriber.ts";
import setAccount from "./set_account.ts";
import getRepoBranchViewportItem from "./get_repo_branch_model_item_prop.ts";
import setOpenRepo from "./set_open_repo.ts";
import loadModel from "./load_model.ts";
import upsertModelNodeProperties from "./upsert_model_node_properties.ts";
import saveUpdates from "./save_updates.ts";
import { TrackedViewportItem } from "tauri-plugin-editor";
// import removeViewport from "./remove_viewport.ts";

declare var describe: any;
declare var it: any;

describe("Update an aws-partition's properties", () => {
    let accountId: string;

    it("get subscriber", async () => {
        let result = await getSubscriber();
        accountId = result.accountId;
    });

    let accountSetRes: AccountDetail;

    it("set account", async () => {
        accountSetRes = await setAccount(accountId);
    });

    let testRepoId: string;
    let testBranchId: string;
    let testVpItemId: string;
    let testVpItemModelItemId: string;
    let testModelItemPropName: string;

    it("get first repo with a branch that has an aws-partition viewport item", async () => {
        let {
            repoId,
            branchId,
            modelItemPropName,
            vpItemId,
            vpItemModelItemId,
        } = await getRepoBranchViewportItem(accountSetRes);
        testRepoId = repoId;
        testBranchId = branchId;
        testVpItemModelItemId = vpItemModelItemId;
        testVpItemId = vpItemId;
        testModelItemPropName = modelItemPropName;
    });

    it("set test repo as opened repo", async () => {
        await setOpenRepo(testRepoId);
    });

    it("set test branch as loaded model", async () => {
        await loadModel(testBranchId);
    });

    it("upsert model node properties", async () => {
        let randVal = new Date().getTime().toString();
        let newProperties = {
            [testModelItemPropName]: `val-${randVal}`,
        };

        await upsertModelNodeProperties(testVpItemModelItemId, newProperties);
    });

    it("save viewport and model property changes", async () => {
        let randVal = new Date().getTime().toString();

        let saveUpdatesReq: TrackedViewportItem[] = [
            {
                vpId: testVpItemId,
                mId: testVpItemModelItemId,
                vpDelta: {
                    upsertedProperties: {
                        randomProp: `val1-${randVal}`,
                        toRemoveProp: "vpPropValToRemove",
                    },
                    removedProperties: ["toRemoveProp"],
                },
                mDelta: {
                    upsertedProperties: {
                        [testModelItemPropName]: `val2-${randVal}`,
                        toRemoveProp: "modelPropValToRemove",
                    },
                    removedProperties: ["toRemoveProp"],
                },
            },
        ];
        let nonUpsertedVpItems: string[] = [];

        await saveUpdates(saveUpdatesReq, nonUpsertedVpItems);
    });
});
