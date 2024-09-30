import passwordLogin from "./password_login.ts";
import mfaLogin from "./mfa_login.ts";

declare var describe: any;
declare var it: any;

describe("Login into the application", () => {
    let session: string;
    let username: string;

    it("password login", async () => {
        let result = await passwordLogin();

        session = result.session;
        username = result.username;
    });

    it("mfa login", async () => {
        await mfaLogin(username, session);
    });
});
