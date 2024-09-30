import {
    AccountDetail,
    CreateRepoRequest,
    commands as accountCommands,
    events as accountEvents,
} from "tauri-plugin-account-client";

export default (
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

    const payload: CreateRepoRequest = {
        reqId: "createRepoRequest",
        parentFolderId: parentFolderId,
    };

    accountCommands
        .createRepo(payload)
        .then((_response) => {
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
