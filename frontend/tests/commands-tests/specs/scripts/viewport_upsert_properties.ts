import {
    ViewportMeta,
    ViewportUpsertPropertiesRequest,
    commands as editorCommands,
    events as editorEvents,
} from "tauri-plugin-editor";

export default (
    viewportId: string,
    properties: any,
    done: (result: { [x: string]: ViewportMeta } | string) => void
) => {
    editorEvents.upsertedViewportsEvent.once((eventData) => {
        console.log("Upsertd viewport items:");
        console.log(JSON.stringify(eventData.payload.viewports, null, 2));
        done(eventData.payload.viewports);
    });

    const payload: ViewportUpsertPropertiesRequest = {
        reqId: "upsertViewportPropertiesRequest",
        viewportId,
        properties,
    };

    editorCommands
        .viewportUpsertProperties(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
