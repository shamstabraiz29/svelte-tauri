import {
    ViewportItemMeta,
    ModelNodeUpsertPropertiesRequest,
    commands as editorCommands,
    events as editorEvents,
} from "tauri-plugin-editor";

export default (
    modelItemId: string,
    properties: any,
    done: (result: { [x: string]: ViewportItemMeta[] } | string) => void
) => {
    editorEvents.upsertedViewportItemsEvent.once((eventData) => {
        done(eventData.payload.viewport_hash_items);
    });

    const payload: ModelNodeUpsertPropertiesRequest = {
        reqId: "upsertViewportItemPropertiesRequest",
        modelItemId,
        properties,
    };

    editorCommands
        .modelNodeUpsertProperties(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
