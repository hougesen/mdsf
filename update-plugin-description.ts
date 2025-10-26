import fs from "node:fs/promises";

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

(async () => {
  const tools = await fs.readdir("tools");

  for (const tool of tools) {
    if (tool === "tool.schema.json") continue;

    const p = `tools/${tool}/plugin.json`;

    const f: {
      description: string | undefined | null;
      homepage: string;
    } = await fs.readFile(p, "utf8").then((b) => JSON.parse(b));

    const homepage = new URL(f.homepage);

    if (homepage.host === "github.com") {
      const response = await fetch(
        `https://api.github.com/repos${homepage.pathname}`,
        {
          headers: {
            "User-Agent": "Awesome-Octocat-App",
            authorization: "Bearer ",
          },
        },
      ).then((res) => res.json());

      console.info(response);

      f.description = (response?.description || f?.description)?.trim();

      if (f?.description?.endsWith(".")) {
        f.description = f?.description
          .slice(0, f.description.length - 1)
          ?.trim();
      }

      f.description = f?.description?.trim() || null;

      await fs.writeFile(p, JSON.stringify(f, undefined, 2));

      await sleep(2500);
    }
  }

  process.exit(0);
})();
