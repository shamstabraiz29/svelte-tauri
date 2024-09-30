import {
    AccountDetail,
    MoveRepoRequest,
    commands as accountCommands,
    events as accountEvents,
} from "tauri-plugin-account-client";

export default (
    repoToMoveId: string,
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

    const payload: MoveRepoRequest = {
        reqId: "moveRepoRequest",
        repoId: repoToMoveId,
        parentFolderId: parentFolderId,
    };

    accountCommands
        .moveRepo(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
