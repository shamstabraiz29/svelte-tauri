import {
    commands as editorCommands,
    LoadModelRequest,
    events as editorEvents,
    BranchDetail,
} from "tauri-plugin-editor";

export default (
    branchId: string,
    done: (result: BranchDetail | string) => void
) => {
    editorEvents.branchDetailEvent.once((eventData) => {
        if (eventData.payload.type === "branch") {
            let branchDetails: BranchDetail = eventData.payload.branchDetail;
            console.log("Branch details:");
            console.log(JSON.stringify(branchDetails, null, 2));
            done(branchDetails);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });

    const payload: LoadModelRequest = {
        reqId: "repoRequest",
        branchId,
        address: "$HEAD",
    };

    editorCommands
        .loadModel(payload)
        .then((_response) => {
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
