import assert from "assert";

import { AccountDetail } from "tauri-plugin-account-client";
import {
    DropInfoRequest,
    PropertiesValuesRequest,
    PropertyValueRequestResponse,
} from "tauri-plugin-editor";

import getSubscriber from "./get_subscriber.ts";
import setAccount from "./set_account.ts";
import getRepo from "./get_repo.ts";
import setOpenRepo from "./set_open_repo.ts";
import loadModel from "./load_model.ts";
import createViewport from "./create_viewport.ts";
import getCloudPatterns from "./get_cloud_patterns.ts";
import doubleClickCloudPattern from "./double_click_cloud_pattern.ts";
import cloudPatternDropInfo from "./cloud_pattern_drop_info.ts";
import cloudPatternPropertiesValues from "./cloud_pattern_properties_values.ts";

declare var describe: any;
declare var it: any;

describe("Drop CloudPattern: 'aws-iam-policy-in-account'", () => {
    let accountId: string;

    it("get subscriber", async () => {
        let result = await getSubscriber();
        accountId = result.accountId;
    });

    let accountSetRes: AccountDetail;

    it("set account", async () => {
        accountSetRes = await setAccount(accountId);
    });

    let testRepo1Id: string;
    let testBranch1Id: string;
    let testRepo2Id: string;
    let testBranch2Id: string;

    it("get first repo with a branch", async () => {
        let getRepoResult = await getRepo(accountSetRes);
        testRepo1Id = getRepoResult.testRepo1Id;
        testBranch1Id = getRepoResult.testBranch1Id;
        testRepo2Id = getRepoResult.testRepo2Id;
        testBranch2Id = getRepoResult.testBranch2Id;
    });

    it("set open repo", async () => {
        await setOpenRepo(testRepo1Id);
    });

    it("load a model (test branch 1)", async () => {
        await loadModel(testBranch1Id);
    });

    it("load a second model (test branch 1)", async () => {
        await loadModel(testBranch1Id);
    });

    it("set repo 2 as open repo", async () => {
        await setOpenRepo(testRepo2Id);
    });

    it("load a model (test branch 2)", async () => {
        await loadModel(testBranch2Id);
    });

    let testViewportId: string;

    it("create a viewport", async () => {
        testViewportId = await createViewport();
        await new Promise((resolve) => setTimeout(resolve, 5000));
    });

    it("get cloud patterns", async () => {
        await getCloudPatterns();
    });

    let dropInfoRequest: DropInfoRequest;
    let cloudPatternId = "aws_iam_policy_in_account_cp";

    it("double-click on a cloud pattern", async () => {
        dropInfoRequest = await doubleClickCloudPattern(cloudPatternId);
    });

    let propertiesValuesRequest: PropertiesValuesRequest;

    it("provide drop information to cloud pattern", async () => {
        propertiesValuesRequest = await cloudPatternDropInfo(
            dropInfoRequest,
            testViewportId
        );
    });

    it("provide drop requested property value to cloud pattern", async () => {
        let propertiesValues: PropertyValueRequestResponse[] = [
            {
                inContextName:
                    propertiesValuesRequest.requests[0].inContextName,
                value: "My Partition",
            },
        ];
        let setPropertiesResponse = await cloudPatternPropertiesValues(
            propertiesValues,
            true
        );

        if (setPropertiesResponse === null) {
            assert.fail("Expected a DropInfoRequest!");
        }

        dropInfoRequest = setPropertiesResponse;
    });

    it("provide drop information to cloud pattern", async () => {
        propertiesValuesRequest = await cloudPatternDropInfo(
            dropInfoRequest,
            testViewportId
        );
    });

    it("provide drop requested property value to cloud pattern", async () => {
        let propertiesValues: PropertyValueRequestResponse[] = [
            {
                inContextName:
                    propertiesValuesRequest.requests[0].inContextName,
                value: "123456789012",
            },
        ];
        let setPropertiesResponse = await cloudPatternPropertiesValues(
            propertiesValues,
            true
        );
        if (setPropertiesResponse === null) {
            assert.fail("Expected a DropInfoRequest!");
        }

        dropInfoRequest = setPropertiesResponse;
    });

    it("provide drop information to cloud pattern", async () => {
        propertiesValuesRequest = await cloudPatternDropInfo(
            dropInfoRequest,
            testViewportId
        );
        assert.equal(propertiesValuesRequest.requests.length, 2);
    });

    it("provide requested properties values to cloud pattern", async () => {
        let propertiesValues: PropertyValueRequestResponse[] = [
            {
                inContextName:
                    propertiesValuesRequest.requests[0].inContextName,
                value: "My Policy",
            },
            {
                inContextName:
                    propertiesValuesRequest.requests[1].inContextName,
                value: "{}",
            },
        ];
        let setPropertiesResponse = await cloudPatternPropertiesValues(
            propertiesValues,
            false
        );
        if (setPropertiesResponse !== null) {
            assert.fail("Expected null!");
        }
    });
});
