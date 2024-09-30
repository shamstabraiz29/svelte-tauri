import {
    commands as repoCommands,
    RepoSetRequest,
    events as repoEvents,
    RepoDetail,
} from "tauri-plugin-repo-client";

export default (
    repoId: string,
    done: (result: RepoDetail | string) => void
) => {
    repoEvents.repoDetailEvent.once((eventData) => {
        if (eventData.payload.type === "repo") {
            done(eventData.payload.repoDetail);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });

    const payload: RepoSetRequest = {
        reqId: "repoRequest",
        repoId,
    };

    repoCommands
        .setRepo(payload)
        .then((response) => {
            console.log(response);
        })
        .catch((error) => {
            done(error);
        });
};
