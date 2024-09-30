import {
    commands as repoCommands,
    RepoRequest,
} from "tauri-plugin-repo-client";

export default (repoId: string, done: (result: any) => void) => {
    const payload: RepoRequest = {
        reqId: "repoRequest",
        repoId,
    };

    repoCommands
        .getRepo(payload)
        .then((response) => {
            done(response);
        })
        .catch((error) => {
            done(error);
        });
};
