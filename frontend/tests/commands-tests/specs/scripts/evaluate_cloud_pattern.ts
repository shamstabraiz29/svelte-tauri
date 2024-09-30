import {
    EvaluateCloudPatternRequest,
    commands as editorCommands,
    events as editorEvents,
    DropInfoRequest,
    PropertiesValuesRequest,
    ViewportItemMeta,
} from "tauri-plugin-editor";

type EvalCloudPatternResponse =
    | { [x: string]: ViewportItemMeta[] }
    | string
    | DropInfoRequest
    | PropertiesValuesRequest;

export default (
    payload: EvaluateCloudPatternRequest,
    composed: boolean,
    done: (result: EvalCloudPatternResponse) => void
) => {
    if (!composed) {
        editorEvents.upsertedViewportItemsEvent.once((eventData) => {
            console.log("Upserted viewport items:");
            console.log(
                JSON.stringify(eventData.payload.viewport_hash_items, null, 2)
            );
            done(eventData.payload.viewport_hash_items);
        });
    }

    editorCommands
        .evaluateCloudPattern(payload)
        .then((response) => {
            if (response.status === "ok") {
                if (response.data.evalResult.type !== "complete") {
                    done(response.data.evalResult);
                }
            } else if (response.status === "error") {
                done(response.error.message);
            }
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
