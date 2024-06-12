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
  const languageHeading = "Formatter";
  let formatterWidth = languageHeading.length;

  const formatterHeading = "Description";
  let descriptionWidth = formatterHeading.length;

  /** @type {Map<string, string>} */
  const formatters = new Map();

  for (const entry of schema.definitions.Tooling.oneOf) {
    const formatter = entry.enum[0];

    const link = `[${entry.description}](${entry.description})`;

    formatterWidth = Math.max(formatterWidth, formatter.length);
    descriptionWidth = Math.max(descriptionWidth, link.length);

    formatters.set(formatter, link);
  }

  /** @type {string[]} */
  const lines = [];

  for (const [key, value] of formatters) {
    const line = `| ${key.padEnd(formatterWidth, " ")} | ${value.padEnd(
      descriptionWidth,
      " ",
    )} |`;

    lines.push(line);
  }

  lines.sort();

  const filler = `| ${"".padEnd(formatterWidth, "-")} | ${"".padEnd(
    descriptionWidth,
    "-",
  )} |`;

  lines.unshift(filler);

  const heading = `| ${languageHeading.padEnd(
    formatterWidth,
    " ",
  )} | ${formatterHeading.padEnd(descriptionWidth, " ")} |`;

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
