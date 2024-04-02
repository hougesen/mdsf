import fs from "fs/promises";

async function getVersion() {
  const content = await fs
    .readFile("Cargo.toml")
    .then((data) => data.toString());

  const line = content.split("\n").find((l) => l.startsWith("version ="));

  return line.split('"')[1];
}

/**
 * @param version {string}
 */
function loadSchema(version) {
  return fs
    .readFile(`schemas/v${version}/mdsf.schema.json`)
    .then((data) => data.toString())
    .then(JSON.parse)
    .then((data) => {
      if (!data || typeof data !== "object") {
        throw new Error("Expected schema to be an object");
      }

      return data;
    });
}

/**
 * @param schema {Awaited<ReturnType<typeof loadSchema>>}
 */
function createLanguageTable(schema) {
  const languageHeading = "Language";
  let languageWidth = languageHeading.length;

  const formatterHeading = "Formatters";
  let formatterWidth = formatterHeading.length;

  /** @type {Map<string, string>} */
  const languages = new Map();

  for (const [key, value] of Object.entries(schema.definitions)) {
    if (
      key.startsWith("MdsfFormatter_") ||
      key.startsWith("Lang_") ||
      key.includes("JavaScriptRuntime")
    ) {
      continue;
    }

    const formatterLine = value.enum
      .sort()
      .map((f) => "`" + f + "`")
      .join(", ");

    languageWidth = Math.max(languageWidth, key.length);
    formatterWidth = Math.max(formatterWidth, formatterLine.length);

    languages.set(key, formatterLine);
  }

  /** @type {string[]} */
  const lines = [];

  for (const [key, value] of languages) {
    const line = `| ${key.padEnd(languageWidth, " ")} | ${value.padEnd(
      formatterWidth,
      " ",
    )} |`;

    lines.push(line);
  }

  lines.sort();

  const filler = `| ${"".padEnd(languageWidth, "-")} | ${"".padEnd(
    formatterWidth,
    "-",
  )} |`;

  lines.unshift(filler);

  const heading = `| ${languageHeading.padEnd(
    languageWidth,
    " ",
  )} | ${formatterHeading.padEnd(formatterWidth, " ")} |`;

  lines.unshift(heading);

  return lines.join("\n");
}

/**
 * @param table {string}
 */
async function updateReadme(table) {
  const content = await fs
    .readFile("README.md")
    .then((data) => data.toString());

  const re =
    /(<!-- START_SECTION:supported-languages -->)[^{}]*<!-- END_SECTION:supported-languages -->/gm;
  const update = `<!-- START_SECTION:supported-languages -->\n\n${table}\n\n<!-- END_SECTION:supported-languages -->`;

  await fs.writeFile("README.md", content.replace(re, update));
}

(async () => {
  const version = await getVersion();

  const schema = await loadSchema(version);
  const table = createLanguageTable(schema);

  await updateReadme(table);
})();
