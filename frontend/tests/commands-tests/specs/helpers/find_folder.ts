import { Folder } from "tauri-plugin-account-client";

export default function findFolder(
    folderId: string,
    parentsFolderIds: string[],
    rootFolders: Folder[]
): Folder | null {
    let targetFolder: Folder | null = null;

    // iterate over the parent folder IDs
    // find the parent folder in the root folders
    // if the parent folder is not found, error
    // it the parent folder is found, set root folders to the parent folder's folders and the target folder to the parent folder
    // iterate over the target folder's folders
    // find the folder with an ID that matches the folder ID

    for (let i = 0; i < parentsFolderIds.length; i++) {
        let parentFolder = rootFolders.find(
            (folder) => folder.id === parentsFolderIds[i]
        );

        if (!parentFolder) {
            return null;
        }

        rootFolders = parentFolder.folders;
        targetFolder = parentFolder;
    }

    targetFolder = rootFolders.find((folder) => folder.id === folderId) || null;

    return targetFolder;
}
