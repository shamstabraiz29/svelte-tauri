import {
    commands as editorCommands,
    events as editorEvents,
    CloudPatternsGetRequest,
    UiCloudPatternMeta,
} from "tauri-plugin-editor";

export default (
    done: (
        result:
            | {
                  [key in string]: UiCloudPatternMeta[];
              }
            | string
    ) => void
) => {
    editorEvents.cloudPatternsMetaEvent.once((eventData) => {
        if (eventData.payload.type === "upsert") {
            done(eventData.payload.cloudPatternsMeta);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });
    const payload: CloudPatternsGetRequest = {
        reqId: "get-cloud-patterns",
    };

    editorCommands
        .getCloudPatterns(payload)
        .then((_) => {
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
