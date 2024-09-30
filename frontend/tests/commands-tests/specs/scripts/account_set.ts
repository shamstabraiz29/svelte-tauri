import {
    commands as accountCommands,
    events as accountEvents,
    AccountSetRequest,
    AccountDetail,
} from "tauri-plugin-account-client";

export default (
    accountId: string,
    done: (result: AccountDetail | string) => void
) => {
    accountEvents.acctDetailEvent.once((eventData) => {
        if (eventData.payload.type === "account") {
            let accountDetails: AccountDetail = eventData.payload.accountDetail;
            done(accountDetails);
        } else {
            done("Got event: " + eventData.payload.type);
        }
    });
    const payload: AccountSetRequest = {
        reqId: "3",
        acctId: accountId,
    };

    accountCommands
        .setAccount(payload)
        .then((_) => {
            // Do nothing.
        })
        .catch((error) => {
            done(error.message);
        });
};
