import {
    commands as editorCommands,
    GetResourceItemsViewportDataRequest,
} from "tauri-plugin-editor";

export default (
    viewportType: string,
    resourceItemsTypes: string[],
    done: (result: unknown) => void
) => {
    const payload: GetResourceItemsViewportDataRequest = {
        reqId: "get-resource-items-viewport-data",
        viewportType,
        resourceItemsTypes,
    };

    editorCommands
        .getResourceItemsViewportData(payload)
        .then((response) => {
            if (response.status === "error") {
                done("Error: " + response.error);
            } else {
                done(response.data.viewportData);
            }
        })
        .catch((error) => {
            done(error.message);
        });
};
