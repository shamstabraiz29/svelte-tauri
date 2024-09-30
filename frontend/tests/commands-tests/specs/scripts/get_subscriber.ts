import {
    commands as subscriberCommands,
    SubscriberRequest,
} from "tauri-plugin-subscriber-client";

export default (done: (result: any) => void) => {
    const payload: SubscriberRequest = {
        reqId: "getSubscriberRequest",
    };
    subscriberCommands
        .getSubscriber(payload)
        .then((response) => {
            done(response);
        })
        .catch((error) => {
            done(error);
        });
};
