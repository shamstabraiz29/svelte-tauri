import {
    commands as editorCommands,
    UnloadModelRequest,
    events as editorEvents,
} from "tauri-plugin-editor";

export default (done: (result: null | string) => void) => {
    editorEvents.branchDetailEvent.once((eventData) => {
        if (eventData.payload.type === "Clear") {
            done(null);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });

    const payload: UnloadModelRequest = {
        reqId: "unloadModelRequest",
    };

    editorCommands
        .unloadModel(payload)
        .then((_response) => {
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
