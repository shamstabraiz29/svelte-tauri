import { AccountDetail } from "tauri-plugin-account-client";

import getSubscriber from "./get_subscriber.ts";
import setAccount from "./set_account.ts";
import createFolder from "./create_folder.ts";
import createRepo from "./create_repo.ts";
import renameRepo from "./rename_repo.ts";
import upsertRepoProperties from "./upsert_repo_properties.ts";
import removeRepoProperties from "./remove_repo_properties.ts";
import moveRepo from "./move_repo.ts";
import archiveRepo from "./archive_repo.ts";

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

    it("rename repo", async () => {
        accountSetRes = await renameRepo(testRepoId, "New Repo Name", []);
    });

    it("upsert repo properties", async () => {
        await upsertRepoProperties(
            testRepoId,
            { prop_key: "prop_value", prop_key2: "prop_value2" },
            []
        );
    });

    it("remove repo properties", async () => {
        accountSetRes = await removeRepoProperties(
            testRepoId,
            ["prop_key2"],
            [],
            {
                prop_key: "prop_value",
            }
        );
    });

    let parentFolderId: string;

    it("create folder to move repo to", async () => {
        let createFolderRes = await createFolder(
            accountSetRes.rootFolderId,
            accountSetRes.rootFolders
        );
        accountSetRes = createFolderRes.acctDetail;
        parentFolderId = createFolderRes.folderId;
    });

    let repoToMoveId: string;

    it("create repo to move", async () => {
        let createRepoRes = await createRepo(
            accountSetRes.rootFolderId,
            accountSetRes.rootRepos
        );
        accountSetRes = createRepoRes.acctDetail;
        repoToMoveId = createRepoRes.repoId;
    });

    it("move repo", async () => {
        await moveRepo(repoToMoveId, parentFolderId, [parentFolderId]);
    });

    it("remove repo", async () => {
        accountSetRes = await archiveRepo(repoToMoveId, [parentFolderId]);
    });
});
