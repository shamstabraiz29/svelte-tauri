import { AccountDetail } from "tauri-plugin-account-client";

import getSubscriber from "./get_subscriber.ts";
import setAccount from "./set_account.ts";
import getRepoBranchViewportItem from "./get_repo_branch_viewport_item.ts";
import setOpenRepo from "./set_open_repo.ts";
import loadModel from "./load_model.ts";
import upsertViewportItemProperties from "./upsert_viewport_item_properties.ts";
import removeViewportItemProperties from "./remove_viewport_item_properties.ts";
import removeViewportProperties from "./remove_viewport_properties.ts";
import upsertViewportProperties from "./upsert_viewport_properties.ts";
// import removeViewport from "./remove_viewport.ts";

declare var describe: any;
declare var it: any;

describe("Update viewport and viewport item properties", () => {
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
    let testViewportId: string;
    let testViewportItemId: string;

    it("get first repo with a branch that has a viewport item", async () => {
        let { repoId, branchId, viewportId, viewportItemId } =
            await getRepoBranchViewportItem(accountSetRes);
        testRepoId = repoId;
        testBranchId = branchId;
        testViewportId = viewportId;
        testViewportItemId = viewportItemId;
    });

    it("set test repo as opened repo", async () => {
        await setOpenRepo(testRepoId);
    });

    it("set test branch as loaded model", async () => {
        await loadModel(testBranchId);
    });

    it("upsert viewport properties", async () => {
        let randVal = new Date().getTime().toString();
        let newProperties = {
            prop1: `val1-${randVal}`,
            prop2: `val2-${randVal}`,
            prop3: `val2-${randVal}`,
        };

        await upsertViewportProperties(testViewportId, newProperties);
    });

    it("remove viewport properties", async () => {
        let propsToRemove = ["prop1", "prop2"];

        await removeViewportProperties(testViewportId, propsToRemove);
    });

    it("upsert viewport item properties", async () => {
        let randVal = new Date().getTime().toString();
        let newProperties = {
            prop1: `val1-${randVal}`,
            prop2: `val2-${randVal}`,
            prop3: `val2-${randVal}`,
        };

        await upsertViewportItemProperties(
            testViewportId,
            testViewportItemId,
            newProperties
        );
    });

    it("remove viewport item properties", async () => {
        let propsToRemove = ["prop1", "prop2"];

        await removeViewportItemProperties(
            testViewportId,
            testViewportItemId,
            propsToRemove
        );
    });

    // it("removes viewport", async () => {
    //     await removeViewport(testViewportId);
    // });
});
