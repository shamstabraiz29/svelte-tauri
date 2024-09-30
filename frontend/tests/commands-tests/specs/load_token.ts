import { promises as fs } from "fs";
import path from "path";
import { fileURLToPath } from "url";

// Replicate __dirname functionality in ES module
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Define the file path
const filePath = path.join(__dirname, "../../../../ignore/credentials.json");

export interface Credentials {
    username: string;
    password: string;
}

export const loadCredentials = async (): Promise<Credentials> => {
    await fs.access(filePath);
    const data = await fs.readFile(filePath, "utf8");
    const parsedCreds: Credentials = JSON.parse(data);
    return parsedCreds;
};
