const types = require("./data/builtins.types.json");
const fs = require("fs");

const fileContent = Object.entries(types)
  .map(([key, value]) => {
    const { fn_type, latest } = value;
    const md = `\`\`\`nix\n${fn_type}\n\`\`\``;
    return [key, md, latest];
  })
  .sort((a, b) => a[0] < b[0])
  .reduce(
    (acc, [key, md, latest]) =>
      `${acc}### ${key} ${latest ? "✅" : "⚠️"}\n\n${md}\n\n`,
    ""
  );

fs.writeFile("./docs/src/generated/builtin-types.md", fileContent, (err) => {
  if (err) {
    console.error(err);
  }
});
