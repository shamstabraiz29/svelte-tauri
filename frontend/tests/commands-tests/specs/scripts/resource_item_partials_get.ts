import {
    commands as editorCommands,
    GetResourceItemPartialsRequest,
} from "tauri-plugin-editor";

export default (
    viewportType: string,
    resourceItemType: string,
    done: (result: unknown) => void
) => {
    const payload: GetResourceItemPartialsRequest = {
        reqId: "get-resource-item-partials",
        viewportType,
        resourceItemType,
    };

    editorCommands
        .getResourceItemPartials(payload)
        .then((response) => {
            if (response.status === "error") {
                done("Error: " + response.error);
            } else {
                done(response.data.partials);
            }
        })
        .catch((error) => {
            done(error.message);
        });
};
