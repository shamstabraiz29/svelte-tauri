import {
    ViewportRemoveRequest,
    commands as editorCommands,
    events as editorEvents,
} from "tauri-plugin-editor";

export default (
    viewportId: string,
    done: (result: string[] | string) => void
) => {
    editorEvents.removedViewportsEvent.once((eventData) => {
        console.log("Replaced viewport items:");
        console.log(
            JSON.stringify(eventData.payload.removed_viewports_ids, null, 2)
        );
        done(eventData.payload.removed_viewports_ids);
    });

    const payload: ViewportRemoveRequest = {
        reqId: "replaceViewportPropertiesRequest",
        viewportId,
    };

    editorCommands
        .viewportRemove(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
