import {
    commands as editorCommands,
    SetTestModelRequest,
    events as editorEvents,
    ViewportItemMeta,
} from "tauri-plugin-editor";

export default (
    viewportId: string,
    done: (result: { [x: string]: ViewportItemMeta[] } | string) => void
) => {
    editorEvents.upsertedViewportItemsEvent.once((eventData) => {
        console.log("Created test model items:");
        console.log(
            JSON.stringify(eventData.payload.viewport_hash_items, null, 2)
        );
        done(eventData.payload.viewport_hash_items);
    });

    const payload: SetTestModelRequest = {
        reqId: "setTestModelRequest",
        viewportId,
    };

    editorCommands
        .setTestModel(payload)
        .then((_response) => {
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
