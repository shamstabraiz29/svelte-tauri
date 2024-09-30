import getResourceItemViewportData from "./get_resource_item_viewport_data.ts";
import getResourceItemsViewportData from "./get_resource_items_viewport_data.ts";
import getResourceItemSchema from "./get_resource_item_schema.ts";
import getResourceItemPartials from "./get_resource_item_partials.ts";
import getResourceItemsPartials from "./get_resource_items_partials.ts";

declare var describe: any;
declare var it: any;

describe("APIs to fetch static data", () => {
    it("get resource item viewport data", async () => {
        let aws_partition_infrastructure_viewport_data =
            await getResourceItemViewportData(
                "infrastructure",
                "aws_partition"
            );
        console.log(
            JSON.stringify(aws_partition_infrastructure_viewport_data, null, 2)
        );
    });

    it("get resource items viewport data", async () => {
        let resources_infrastructure_viewport_data =
            await getResourceItemsViewportData("infrastructure", [
                "aws_partition",
                "aws_region",
            ]);
        console.log(
            JSON.stringify(resources_infrastructure_viewport_data, null, 2)
        );
    });

    it("get resource item schema", async () => {
        let aws_partition_schema = await getResourceItemSchema("aws_partition");
        console.log(JSON.stringify(aws_partition_schema, null, 2));
    });

    it("get resource item partials", async () => {
        let aws_partition_infrastructure_partials =
            await getResourceItemPartials("infrastructure", "aws_partition");
        console.log(
            JSON.stringify(aws_partition_infrastructure_partials, null, 2)
        );
    });

    it("get resource items partials", async () => {
        let resources_infrastructure_partials = await getResourceItemsPartials(
            "infrastructure",
            ["aws_partition", "aws_region"]
        );
        console.log(JSON.stringify(resources_infrastructure_partials, null, 2));
    });
});
