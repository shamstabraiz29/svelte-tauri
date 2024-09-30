import {
    ViewportItemMeta,
    SaveUpdatesRequest,
    TrackedViewportItem,
    commands as editorCommands,
    events as editorEvents,
} from "tauri-plugin-editor";

export default (
    viewportItems: TrackedViewportItem[],
    done: (
        result:
            | {
                  [x: string]: ViewportItemMeta[];
              }
            | string
    ) => void
) => {
    editorEvents.upsertedViewportItemsEvent.once((eventData) => {
        console.log("Upserted viewport item:");
        console.log(
            JSON.stringify(eventData.payload.viewport_hash_items, null, 2)
        );
        done(eventData.payload.viewport_hash_items);
    });

    const payload: SaveUpdatesRequest = {
        reqId: "saveUpdatesRequest",
        trackedViewportItems: viewportItems,
    };

    editorCommands
        .saveUpdates(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
