import {
    AccountDetail,
    RemoveFolderRequest,
    commands as accountCommands,
    events as accountEvents,
} from "tauri-plugin-account-client";

export default (
    folderId: string,
    done: (result: AccountDetail | string) => void
) => {
    accountEvents.acctDetailEvent.once((eventData) => {
        if (eventData.payload.type === "account") {
            done(eventData.payload.accountDetail);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });

    const payload: RemoveFolderRequest = {
        reqId: "removeFolderRequest",
        folderId: folderId,
    };

    accountCommands
        .removeFolder(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
