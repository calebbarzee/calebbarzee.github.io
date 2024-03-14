import fs from "fs";

export default function getText(file_path) {
  const file = `${file_path}.txt`;
  const content = fs.readFileSync(file, "utf8");
  const contentString = content.toString();
  // console.log(contentString);
  return contentString;
}
