import fs from "fs";
import matter from "gray-matter";

const folder = "articles/";

const get_metadata = () => {
  const files = fs.readdirSync(folder);
  const markdown_articles = files.filter((file) => file.endsWith(".md"));

  const articles = markdown_articles.map((fileName) => {
    const fileContents = fs.readFileSync(`articles/${fileName}`, "utf8");
    const matterResult = matter(fileContents);
    const metadata = {
      title: matterResult.data.title,
      subtitle: matterResult.data.subtitle,
      date_written: matterResult.data.date_written,
      slug: fileName.replace(".md", ""),
    };
    // console.log(metadata);
    return metadata;
  });

  return articles;
};

const get_content = (slug) => {
  const file = `${folder}${slug}.md`;
  const content = fs.readFileSync(file, "utf8");
  const matterResult = matter(content);
  // console.log(matterResult)
  return matterResult;
};

export { get_metadata, get_content };
