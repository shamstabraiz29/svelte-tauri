import { RepoMeta, AccountDetail } from "tauri-plugin-account-client";

import findFolder from "./find_folder.ts";

export default function findRepo(
    repoId: string,
    parentsFolderIds: string[],
    acctDetail: AccountDetail
): RepoMeta | null {
    if (parentsFolderIds.length === 0) {
        return acctDetail.rootRepos.find((repo) => repo.id === repoId) || null;
    }

    let parentFolder = findFolder(
        parentsFolderIds.slice(-1)[0],
        parentsFolderIds.slice(0, -1),
        acctDetail.rootFolders
    );
    return parentFolder?.repoMetas.find((repo) => repo.id === repoId) || null;
}
