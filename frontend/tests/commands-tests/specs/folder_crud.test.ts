import { AccountDetail } from "tauri-plugin-account-client";

import getSubscriber from "./get_subscriber.ts";
import setAccount from "./set_account.ts";
import createFolder from "./create_folder.ts";
import renameFolder from "./rename_folder.ts";
import upsertFolderProperties from "./upsert_folder_properties.ts";
import removeFolderProperties from "./remove_folder_properties.ts";
import moveFolder from "./move_folder.ts";
import removeFolder from "./remove_folder.ts";

declare var describe: any;
declare var it: any;

describe("Folder modification commands tests", () => {
    let accountId: string;

    it("get subscriber", async () => {
        let result = await getSubscriber();
        accountId = result.accountId;
    });

    let accountSetRes: AccountDetail;

    it("set account", async () => {
        accountSetRes = await setAccount(accountId);
    });

    let testFolderId: string;

    it("create folder", async () => {
        let createFolderRes = await createFolder(
            accountSetRes.rootFolderId,
            accountSetRes.rootFolders
        );
        accountSetRes = createFolderRes.acctDetail;
        testFolderId = createFolderRes.folderId;
    });

    it("rename folder", async () => {
        accountSetRes = await renameFolder(testFolderId, "New Folder Name", []);
    });

    it("upsert folder properties", async () => {
        await upsertFolderProperties(
            testFolderId,
            { prop_key: "prop_value", prop_key2: "prop_value2" },
            []
        );
    });

    it("remove folder properties", async () => {
        accountSetRes = await removeFolderProperties(
            testFolderId,
            ["prop_key2"],
            [],
            {
                prop_key: "prop_value",
            }
        );
    });

    let folderToMoveId: string;

    it("create folder to move", async () => {
        let createFolderRes = await createFolder(
            accountSetRes.rootFolderId,
            accountSetRes.rootFolders
        );
        accountSetRes = createFolderRes.acctDetail;
        folderToMoveId = createFolderRes.folderId;
    });

    it("move folder", async () => {
        await moveFolder(folderToMoveId, testFolderId, [testFolderId]);
    });

    it("remove folder", async () => {
        accountSetRes = await removeFolder(folderToMoveId, [testFolderId]);
    });
});
