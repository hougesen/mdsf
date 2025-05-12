import path from "node:path";
import { addPath, getInput, platform, setFailed } from "@actions/core";
import { downloadTool, extractTar, extractZip } from "@actions/tool-cache";

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

/**
 * @param {string} version
 * @param {string} arch
 */
function getPackageArchiveFormatFromVersion(version, arch) {
  if (
    [
      "v0.0.0",
      "v0.0.1",
      "v0.0.2",
      "v0.0.3",
      "v0.0.4",
      "v0.0.5",
      "v0.0.6",
      "v0.0.7",
      "v0.0.8",
      "v0.1.0",
      "v0.1.1",
      "v0.1.2",
      "v0.2.0",
      "v0.2.1",
      "v0.2.2",
      "v0.2.3",
      "v0.2.4",
      "v0.2.5",
      "v0.2.6",
      "v0.2.7",
      "v0.3.0",
      "v0.3.1",
      "v0.3.2",
      "v0.4.0",
      "v0.4.1",
      "v0.5.0",
      "v0.5.1",
      "v0.5.2",
      "v0.5.3",
      "v0.6.0",
      "v0.6.1",
      "v0.7.0",
      "v0.8.0",
      "v0.8.1",
      "v0.8.2",
      "v0.8.3",
      "v0.8.4",
      "v0.8.5",
      "v0.9.0",
      "v0.9.1",
      "v0.9.2",
    ].includes(version)
  ) {
    return ".tar.gz";
  }

  if (arch.includes("windows")) {
    return ".zip";
  }

  if (version === "v0.9.3") {
    return ".tar.xz";
  }

  return ".tar.gz";
}

/**
 * @param {string} downloadPath
 * @param {ReturnType<typeof getPackageArchiveFormatFromVersion >} archiveFormat
 */
function extractTool(downloadPath, archiveFormat) {
  switch (archiveFormat) {
    case ".zip":
      return extractZip(downloadPath);

    case ".tar.xz":
    case ".tar.gz":
      return extractTar(downloadPath);
  }
}

export async function setup() {
  const version = getInput("version");

  const platformArch = await getArchInfo();

  const file = `mdsf-${platformArch}`;

  const archiveFormat = getPackageArchiveFormatFromVersion(
    version,
    platformArch,
  );

  const downloadPath = await getPackageDownloadPath(
    version,
    `${file}${archiveFormat}`,
  );

  const pathToArchive = await downloadTool(downloadPath);

  const pathToCLI = await extractTool(pathToArchive, archiveFormat);

  addPath(path.join(pathToCLI, file));
}

setup().catch((error) => {
  setFailed(error.message);
});
