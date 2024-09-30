import {
    ViewportItemMeta,
    ViewportItemUpsertPropertiesRequest,
    commands as editorCommands,
    events as editorEvents,
} from "tauri-plugin-editor";

export default (
    viewportItemId: string,
    properties: any,
    done: (result: { [x: string]: ViewportItemMeta[] } | string) => void
) => {
    editorEvents.upsertedViewportItemsEvent.once((eventData) => {
        console.log("Upsertd viewport items:");
        console.log(
            JSON.stringify(eventData.payload.viewport_hash_items, null, 2)
        );
        done(eventData.payload.viewport_hash_items);
    });

    const payload: ViewportItemUpsertPropertiesRequest = {
        reqId: "upsertViewportItemPropertiesRequest",
        viewportItemId,
        properties,
    };

    editorCommands
        .viewportItemUpsertProperties(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
