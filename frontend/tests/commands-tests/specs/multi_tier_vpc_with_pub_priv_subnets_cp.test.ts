// import { AccountDetail } from "tauri-plugin-account-client";
// import { DropInfoRequest } from "tauri-plugin-editor";
// import { RepoDetail } from "tauri-plugin-repo-client";

// import getSubscriber from "./get_subscriber.ts";
// import setAccount from "./set_account.ts";
// import createRepo from "./create_repo.ts";
// import createBranch from "./create_branch.ts";
// import setOpenRepo from "./set_open_repo.ts";
// import loadModel from "./load_model.ts";
// import createViewport from "./create_viewport.ts";
// import getCloudPatterns from "./get_cloud_patterns.ts";
// import doubleClickCloudPattern from "./double_click_cloud_pattern.ts";

// declare var describe: any;
// declare var it: any;

// describe("CloudPattern: Multi-Tier VPC with Public-Private Subnets", () => {
//     let accountId: string;

//     it("get subscriber", async () => {
//         let result = await getSubscriber();
//         accountId = result.accountId;
//     });

//     let accountSetRes: AccountDetail;

//     it("set account", async () => {
//         accountSetRes = await setAccount(accountId);
//     });

//     let testRepoId: string;

//     it("create repo", async () => {
//         let createRepoRes = await createRepo(
//             accountSetRes.rootFolderId,
//             accountSetRes.rootRepos
//         );
//         accountSetRes = createRepoRes.acctDetail;
//         testRepoId = createRepoRes.repoId;
//     });

//     let repoSetRes: RepoDetail;

//     it("set open repo", async () => {
//         repoSetRes = await setOpenRepo(testRepoId);
//     });

//     let testBranchId: string;

//     it("create branch", async () => {
//         let createBranchRes = await createBranch(
//             "New Branch Name",
//             null,
//             {},
//             Object.values(repoSetRes.branches)
//         );
//         repoSetRes = createBranchRes.repoDetail;
//         testBranchId = createBranchRes.branchId;
//     });

//     it("load a model (test branch 1)", async () => {
//         await loadModel(testBranchId);
//     });

//     let testViewportId: string;

//     it("create a viewport", async () => {
//         testViewportId = await createViewport();
//     });

//     it("get cloud patterns", async () => {
//         await getCloudPatterns();
//     });

//     let dropInfoRequest: DropInfoRequest;
//     let cloudPatternId = "aws_account_in_partition_cp";

//     it("double-click on a cloud pattern", async () => {
//         dropInfoRequest = await doubleClickCloudPattern(cloudPatternId);
//     });
// });
