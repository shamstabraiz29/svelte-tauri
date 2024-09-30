import { AccountDetail } from "tauri-plugin-account-client";

import getSubscriber from "./get_subscriber.ts";
import setAccount from "./set_account.ts";
import createRepo from "./create_repo.ts";
import renameBranch from "./rename_branch.ts";
import upsertBranchProperties from "./upsert_branch_properties.ts";
import removeBranchProperties from "./remove_branch_properties.ts";
import removeBranch from "./remove_branch.ts";
import createBranch from "./create_branch.ts";
import setOpenRepo from "./set_open_repo.ts";
import { RepoDetail } from "tauri-plugin-repo-client";

declare var describe: any;
declare var it: any;

describe("Repo modification commands tests", () => {
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

    it("rename repo", async () => {
        repoSetRes = await renameBranch(testBranchId, "Renamed Branch Name");
    });

    it("upsert repo properties", async () => {
        repoSetRes = await upsertBranchProperties(testBranchId, {
            prop_key: "prop_value",
            prop_key2: "prop_value2",
        });
    });

    it("remove repo properties", async () => {
        repoSetRes = await removeBranchProperties(testBranchId, ["prop_key2"], {
            prop_key: "prop_value",
        });
    });

    it("remove branch", async () => {
        repoSetRes = await removeBranch(testBranchId);
    });
});
