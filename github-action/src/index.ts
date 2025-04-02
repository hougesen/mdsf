import core from "@actions/core";
import tc from "@actions/tool-cache";

// https://github.com/actions/toolkit/blob/main/packages/core/README.md#platform-helper
async function getArchInfo() {
  const details = await core.platform.getDetails();

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

function parseVersion(version: string) {
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

async function getPackageDownloadPath(version: string) {
  const platformArch = await getArchInfo();

  const file = `mdsf-${platformArch}.tar.gz`;

  const parsedVersion = parseVersion(version);

  if (parsedVersion?.length) {
    return `https://github.com/hougesen/mdsf/releases/download/${
      parsedVersion
    }/${file}`;
  }

  return `https://github.com/hougesen/mdsf/releases/latest/download/${file}`;
}

export default async function setup() {
  const version = core.getInput("version");

  const downloadPath = await getPackageDownloadPath(version);

  const pathToTarball = await tc.downloadTool(downloadPath);

  const pathToCLI = await tc.extractTar(pathToTarball);

  core.addPath(pathToCLI);
}
