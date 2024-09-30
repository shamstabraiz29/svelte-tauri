import {
    ViewportItemMeta,
    ViewportItemRemovePropertiesRequest,
    commands as editorCommands,
    events as editorEvents,
} from "tauri-plugin-editor";

export default (
    viewportItemId: string,
    properties: string[],
    done: (result: { [x: string]: ViewportItemMeta[] } | string) => void
) => {
    editorEvents.upsertedViewportItemsEvent.once((eventData) => {
        console.log("Removed viewport items:");
        console.log(
            JSON.stringify(eventData.payload.viewport_hash_items, null, 2)
        );
        done(eventData.payload.viewport_hash_items);
    });

    const payload: ViewportItemRemovePropertiesRequest = {
        reqId: "removeViewportItemPropertiesRequest",
        viewportItemId,
        properties,
    };

    editorCommands
        .viewportItemRemoveProperties(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
