import {
    RepoDetail,
    CreateBranchRequest,
    commands as repoCommands,
    events as repoEvents,
    ParentBranchPointDetail,
} from "tauri-plugin-repo-client";

export default (
    name: string,
    parentBranch: ParentBranchPointDetail | null,
    properties: { [x: string]: unknown },
    done: (result: RepoDetail | string) => void
) => {
    repoEvents.repoDetailEvent.once((eventData) => {
        if (eventData.payload.type === "repo") {
            done(eventData.payload.repoDetail);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });

    const payload: CreateBranchRequest = {
        reqId: "createBranchRequest",
        name,
        parentBranch,
        properties,
    };

    repoCommands
        .createBranch(payload)
        .then((_response) => {
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
