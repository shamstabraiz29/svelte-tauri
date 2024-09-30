import {
    AccountDetail,
    UpsertRepoPropertiesRequest,
    commands as accountCommands,
    events as accountEvents,
} from "tauri-plugin-account-client";

export default (
    repoId: string,
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

    const payload: UpsertRepoPropertiesRequest = {
        reqId: "upsertRepoPropertiesRequest",
        repoId,
        properties,
    };

    accountCommands
        .upsertRepoProperties(payload)
        .then((response) => {
            console.log(response);
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
