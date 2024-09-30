import { nodeResolve } from "@rollup/plugin-node-resolve";
import typescript from "@rollup/plugin-typescript";
import terser from "@rollup/plugin-terser";

export default {
    input: "./bindings/index.mts", // Entry point for your application
    output: {
        dir: "./dist", // Output directory for the build files
        entryFileNames: "[name].mjs", // Naming pattern for the output files
        format: "es", // Module format (ES modules in this case)
        exports: "auto", // Rollup will determine the export mode automatically
    },
    plugins: [
        nodeResolve(), // Resolves node modules
        terser(), // Minifies the output (useful for production)
        typescript({
            tsconfig: "./tsconfig.json", // Points to your tsconfig.json file
            moduleResolution: "bundler",
            declaration: true, // Generates declaration files (.d.ts)
        }),
    ],
};
