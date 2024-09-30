import {
    commands as editorCommands,
    GetResourceItemSchemaRequest,
    ResourceItemSchemaDto,
} from "tauri-plugin-editor";

export default (
    resourceItemType: string,
    done: (result: ResourceItemSchemaDto | string) => void
) => {
    const payload: GetResourceItemSchemaRequest = {
        reqId: "get-resource-item-schema",
        resourceItemType,
    };

    editorCommands
        .getResourceItemSchema(payload)
        .then((response) => {
            if (response.status === "error") {
                done("Error: " + response.error);
            } else {
                done(response.data.schema);
            }
        })
        .catch((error) => {
            done(error.message);
        });
};
