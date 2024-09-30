import { AccountDetail } from "tauri-plugin-account-client";

import getSubscriber from "./get_subscriber.ts";
import setAccount from "./set_account.ts";
import loadModel from "./load_model.ts";
import createRepo from "./create_repo.ts";
import createViewport from "./create_viewport.ts";
import createTestModel from "./create_test_model.ts";
import createBranch from "./create_branch.ts";
import setOpenRepo from "./set_open_repo.ts";
import { RepoDetail } from "tauri-plugin-repo-client";

declare var describe: any;
declare var it: any;

describe("Set up a test model in a new repo and branch", () => {
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

    it("create repo", async () => {
        let createRepoRes = await createRepo(
            accountSetRes.rootFolderId,
            accountSetRes.rootRepos
        );
        accountSetRes = createRepoRes.acctDetail;
        testRepoId = createRepoRes.repoId;
    });

    let repoSetRes: RepoDetail;

    it("set open repo", async () => {
        repoSetRes = await setOpenRepo(testRepoId);
    });

    let testBranchId: string;

    it("create branch", async () => {
        let createBranchRes = await createBranch(
            "New Branch Name",
            null,
            {},
            Object.values(repoSetRes.branches)
        );
        repoSetRes = createBranchRes.repoDetail;
        testBranchId = createBranchRes.branchId;
    });

    it("load created branch as open model", async () => {
        await loadModel(testBranchId);
    });

    let testViewportId: string;

    it("create a viewport", async () => {
        testViewportId = await createViewport();
    });

    it("creates the test model", async () => {
        await createTestModel(testViewportId);
    });

    it("unload open model", async () => {
        await loadModel(testBranchId);
    });

    it("load updated test model", async () => {
        await loadModel(testBranchId);
    });
});
