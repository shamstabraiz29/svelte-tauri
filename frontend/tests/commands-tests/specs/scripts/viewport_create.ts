import {
    UiViewportCreateRequest,
    commands as editorCommands,
    events as editorEvents,
    ViewportMeta,
    ViewportItemMeta,
} from "tauri-plugin-editor";

export default (
    name: string,
    viewportType: string,
    config: any,
    done: (result: { [x: string]: ViewportMeta } | string) => void
) => {
    editorEvents.upsertedViewportsEvent.once((eventData) => {
        if (eventData.payload.type === "UpsertedViewportsEvent") {
            let viewports: { [x: string]: ViewportMeta } =
                eventData.payload.viewports;
            done(viewports);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });

    editorEvents.upsertedViewportItemsEvent.once((eventData) => {
        if (eventData.payload.type === "UpsertedViewportItemsEvent") {
            let viewport_items: {
                [x: string]: ViewportItemMeta[];
            } = eventData.payload.viewport_hash_items;
            console.log("Created viewports items: ", viewport_items);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });

    const payload: UiViewportCreateRequest = {
        reqId: "viewportCreateRequest",
        name,
        viewportType,
        config,
    };

    editorCommands
        .viewportCreate(payload)
        .then((_response) => {
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
