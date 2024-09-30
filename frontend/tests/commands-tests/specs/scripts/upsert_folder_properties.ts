import {
    AccountDetail,
    UpsertFolderPropertiesRequest,
    commands as accountCommands,
    events as accountEvents,
} from "tauri-plugin-account-client";

export default (
    folderId: string,
    properties: any,
    done: (result: AccountDetail | string) => void
) => {
    accountEvents.acctDetailEvent.once((eventData) => {
        if (eventData.payload.type === "account") {
            done(eventData.payload.accountDetail);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });

    const payload: UpsertFolderPropertiesRequest = {
        reqId: "upsertFolderPropertiesRequest",
        folderId,
        properties,
    };

    accountCommands
        .upsertFolderProperties(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
