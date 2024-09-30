import {
    ViewportMeta,
    ViewportRemovePropertiesRequest,
    commands as editorCommands,
    events as editorEvents,
} from "tauri-plugin-editor";

export default (
    viewportId: string,
    properties: string[],
    done: (result: { [x: string]: ViewportMeta } | string) => void
) => {
    editorEvents.upsertedViewportsEvent.once((eventData) => {
        console.log("Removed viewport items:");
        console.log(JSON.stringify(eventData.payload.viewports, null, 2));
        done(eventData.payload.viewports);
    });

    const payload: ViewportRemovePropertiesRequest = {
        reqId: "removeViewportPropertiesRequest",
        viewportId,
        properties,
    };

    editorCommands
        .viewportRemoveProperties(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
