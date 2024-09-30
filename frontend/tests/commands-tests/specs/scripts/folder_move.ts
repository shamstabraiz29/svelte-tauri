import {
    AccountDetail,
    MoveFolderRequest,
    commands as accountCommands,
    events as accountEvents,
} from "tauri-plugin-account-client";

export default (
    folderToMoveId: string,
    parentFolderId: string,
    done: (result: AccountDetail | string) => void
) => {
    accountEvents.acctDetailEvent.once((eventData) => {
        if (eventData.payload.type === "account") {
            done(eventData.payload.accountDetail);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });

    const payload: MoveFolderRequest = {
        reqId: "moveFolderRequest",
        folderId: folderToMoveId,
        parentFolderId: parentFolderId,
    };

    accountCommands
        .moveFolder(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
