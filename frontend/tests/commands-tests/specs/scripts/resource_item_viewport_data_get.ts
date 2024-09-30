import {
    commands as editorCommands,
    GetResourceItemViewportDataRequest,
} from "tauri-plugin-editor";

export default (
    viewportType: string,
    resourceItemType: string,
    done: (result: unknown) => void
) => {
    const payload: GetResourceItemViewportDataRequest = {
        reqId: "get-resource-item-viewport-data",
        viewportType,
        resourceItemType,
    };

    editorCommands
        .getResourceItemViewportData(payload)
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
