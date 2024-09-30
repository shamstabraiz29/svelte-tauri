import {
    commands as editorCommands,
    GetResourceItemsPartialsRequest,
} from "tauri-plugin-editor";

export default (
    viewportType: string,
    resourceItemsTypes: string[],
    done: (result: unknown) => void
) => {
    const payload: GetResourceItemsPartialsRequest = {
        reqId: "get-resource-items-partials",
        viewportType,
        resourceItemsTypes,
    };

    editorCommands
        .getResourceItemsPartials(payload)
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
