import fs from "node:fs/promises";
import path from "node:path";
import { addPath, getInput, platform, setFailed } from "@actions/core";
import { downloadTool, extractTar } from "@actions/tool-cache";

// https://github.com/actions/toolkit/blob/main/packages/core/README.md#platform-helper
async function getArchInfo() {
  const details = await platform.getDetails();

  if (details.isLinux) {
    if (details?.arch === "x64") {
      return "x86_64-unknown-linux-gnu";
    }
  }

  if (details.isWindows) {
    if (details?.arch === "x64") {
      return "x86_64-pc-windows-msvc";
    }
  }

  if (details.isMacOS) {
    if (details?.arch === "x64") {
      return "x86_64-apple-darwin";
    }

    if (details?.arch === "arm64") {
      return "aarch64-apple-darwin";
    }
  }

  throw new Error(
    `Unsupported platform '${details.platform}' '${details.arch}'`,
  );
}

/**
 * @param {string} version
 */
function parseVersion(version) {
  const trimmed = version?.trim();

  if (!trimmed?.length) {
    return null;
  }

  if (trimmed === "latest") {
    return null;
  }

  if (version?.at(0) === "v") {
    return version;
  }

  return `v${version}`;
}

/**
 * @param {string} version
 * @param {string} file
 */
async function getPackageDownloadPath(version, file) {
  const parsedVersion = parseVersion(version);

  if (parsedVersion?.length) {
    return `https://github.com/hougesen/mdsf/releases/download/${
      parsedVersion
    }/${file}`;
  }

  return `https://github.com/hougesen/mdsf/releases/latest/download/${file}`;
}

export async function setup() {
  const version = getInput("version");

  const platformArch = await getArchInfo();

  const file = `mdsf-${platformArch}.tar.gz`;

  const downloadPath = await getPackageDownloadPath(version, file);

  const pathToTarball = await downloadTool(downloadPath);

  const pathToCLI = await extractTar(pathToTarball);

  console.info("pathToCLI", pathToCLI);

  const contents = await fs.readdir(pathToCLI);

  console.info("contents", contents);

  addPath(path.join(pathToCLI, file));
}

setup().catch((error) => {
  setFailed(error.message);
});
