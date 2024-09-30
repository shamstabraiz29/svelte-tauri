import {
    RepoDetail,
    RemoveBranchRequest,
    commands as repoCommands,
    events as repoEvents,
} from "tauri-plugin-repo-client";

export default (
    branchId: string,
    done: (result: RepoDetail | string) => void
) => {
    repoEvents.repoDetailEvent.once((eventData) => {
        if (eventData.payload.type === "repo") {
            done(eventData.payload.repoDetail);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });

    const payload: RemoveBranchRequest = {
        reqId: "removeBranchRequest",
        branchId: branchId,
    };

    repoCommands
        .removeBranch(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
