import fs from "fs";
const readFile = fs.promises.readFile;

async function main() {
  const file = process.argv[2];
  const code = (await readFile(file)).toString();
  console.log(code);
  eval(code);
  // console.error("true\n    !=\nfalse");
  // process.exit(1);
}

main();
