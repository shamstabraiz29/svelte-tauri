import {
    commands as loginCommands,
    MfaLoginRequest,
} from "tauri-plugin-cognito-login";

export default (
    username: string,
    mfaCode: string,
    session: string,
    done: (result: any) => void
) => {
    const payload: MfaLoginRequest = {
        reqId: "2",
        username,
        mfaCode,
        session,
    };

    loginCommands
        .mfaLogin(payload)
        .then((response) => {
            done(response);
        })
        .catch((error) => {
            done(error);
        });
};
