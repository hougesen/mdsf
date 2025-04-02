"use strict";
var __importDefault =
  (this && this.__importDefault) ||
  function (mod) {
    return mod && mod.__esModule ? mod : { default: mod };
  };
Object.defineProperty(exports, "__esModule", { value: true });
exports.default = setup;
const core_1 = __importDefault(require("@actions/core"));
const tool_cache_1 = __importDefault(require("@actions/tool-cache"));
async function getArchInfo() {
  const details = await core_1.default.platform.getDetails();
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
async function getPackageDownloadPath(version) {
  const platformArch = await getArchInfo();
  const file = `mdsf-${platformArch}.tar.gz`;
  const parsedVersion = parseVersion(version);
  if (parsedVersion?.length) {
    return `https://github.com/hougesen/mdsf/releases/download/${parsedVersion}/${file}`;
  }
  return `https://github.com/hougesen/mdsf/releases/latest/download/${file}`;
}
async function setup() {
  const version = core_1.default.getInput("version");
  const downloadPath = await getPackageDownloadPath(version);
  const pathToTarball = await tool_cache_1.default.downloadTool(downloadPath);
  const pathToCLI = await tool_cache_1.default.extractTar(pathToTarball);
  core_1.default.addPath(pathToCLI);
}
