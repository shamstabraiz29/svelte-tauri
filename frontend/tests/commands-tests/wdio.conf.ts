import type { Options } from "@wdio/types";
import os from "os";
import path from "path";
import { spawn, spawnSync } from "child_process";

import { loadCredentials } from "./specs/load_token.ts";

let tauriDriver: ReturnType<typeof spawn>;
let browser: any;

export const config: Options.Testrunner = {
    hostname: "localhost", // Specify the hostname
    port: 4444, // Specify the port
    autoCompileOpts: {
        autoCompile: true,
        tsNodeOpts: {
            project: "./tsconfig.json",
            transpileOnly: true,
        },
    },
    specs: [
        [
            "./specs/login.test.ts",
            "./specs/static_data_fetch.test.ts",
            "./specs/folder_crud.test.ts",
            "./specs/set_test_model.test.ts",
            "./specs/repo_crud.test.ts",
            "./specs/branch_crud.test.ts",
            "./specs/aws_partition_cp.test.ts",
            "./specs/aws_partition_update.test.ts",
            "./specs/aws_account_in_partition_cp.test.ts",
            "./specs/aws_iam_policy_in_account_cp.test.ts",
            "./specs/vp_vpi_rud.test.ts",
            "./specs/close_tauri_app.test.ts",
        ],
    ],
    maxInstances: 1,
    maxInstancesPerCapability: 1,
    capabilities: [
        {
            "tauri:options": {
                application: "../../target/debug/commands-tests",
            },
        },
    ],
    logLevel: "trace",
    bail: 0,
    waitforTimeout: 10000,
    connectionRetryTimeout: 240000,
    connectionRetryCount: 0,
    framework: "mocha",
    reporters: ["spec"],
    mochaOpts: {
        ui: "bdd",
        timeout: 120000,
    },

    onPrepare: function (_config, _capabilities) {
        spawnSync("cargo", ["tauri", "build", "--no-bundle", "--debug"], {});
    },
    beforeSession: () =>
        (tauriDriver = spawn(
            path.resolve(os.homedir(), ".cargo", "bin", "tauri-driver"),
            [],
            { stdio: [null, process.stdout, process.stderr] }
        )),
    before: (_capabilities, _spec, brswr) => {
        browser = brswr;

        browser.addCommand("loadCredentials", async function () {
            let credentials = await loadCredentials();
            return credentials;
        });
    },
    afterSession: () => {
        tauriDriver.kill();

        const isTauriDriverRunning =
            spawnSync("pgrep", ["-f", "tauri-driver"]).stdout.length > 0;
        if (isTauriDriverRunning) {
            spawnSync("pkill", ["-f", "tauri-driver"]);
        }
    },
    onComplete: () => {
        const isTauriDriverRunning =
            spawnSync("pgrep", ["-f", "tauri-driver"]).stdout.length > 0;
        if (isTauriDriverRunning) {
            spawnSync("pkill", ["-f", "tauri-driver"]);
        }
    },
};
