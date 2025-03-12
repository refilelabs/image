/// <reference types="bun-types" />
import { readPackageJSON, writePackageJSON } from "pkg-types";

// Update the package.json file to have the correct name

const packageJsonPath = import.meta.dir + '/../pkg/package.json';

const packageJsonFile = Bun.file(packageJsonPath);

if (!packageJsonFile.exists()) {
  throw new Error('package.json file not found');
}

const packageJson = await readPackageJSON(packageJsonPath);
packageJson.name = '@refilelabs/image';
await writePackageJSON(packageJsonPath, packageJson);

// Update the readme file to use the js readme

const readmePath = import.meta.dir + '/../pkg/README.md';
const jsReadmePath = import.meta.dir + '/../README_JS.md';

const jsReadmeFile = Bun.file(jsReadmePath);

await Bun.write(readmePath, jsReadmeFile)

console.log('✨ Updated package.json and README.md');