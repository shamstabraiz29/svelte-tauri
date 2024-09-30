import {
    AccountDetail,
    RemoveFolderPropertiesRequest,
    commands as accountCommands,
    events as accountEvents,
} from "tauri-plugin-account-client";

export default (
    folderId: string,
    properties: string[],
    done: (result: AccountDetail | string) => void
) => {
    accountEvents.acctDetailEvent.once((eventData) => {
        if (eventData.payload.type === "account") {
            done(eventData.payload.accountDetail);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });

    const payload: RemoveFolderPropertiesRequest = {
        reqId: "removeFolderPropertiesRequest",
        folderId,
        properties,
    };

    accountCommands
        .removeFolderProperties(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
