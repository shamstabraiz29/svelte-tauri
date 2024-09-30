import {
    commands as loginCommands,
    PwLoginRequest,
} from "tauri-plugin-cognito-login";

export default (
    username: string,
    password: string,
    done: (result: any) => void
) => {
    const payload: PwLoginRequest = {
        reqId: "1",
        email: username,
        password: password,
    };
    loginCommands
        .pwLogin(payload)
        .then((response) => {
            done(response);
        })
        .catch((error) => {
            done(error);
        });
};
