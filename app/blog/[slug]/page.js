import Markdown from "markdown-to-jsx";
import {get_metadata, get_content} from "../../utils/get_article";

export const generateStaticParams = async () => {
  const articles = get_metadata();
  return articles.map((article) => ({
    slug: article.slug,
  }));
};

const ArticlePage = (props) => {
  const slug = props.params.slug;
  const article = get_content(slug);
  return (
    <div>
      <div className="my-12 text-center">
        <h1 className="text-2xl">{article.data.title}</h1>
        <p className="mt-2">{article.data.date}</p>
      </div>

      <article className="prose">
        <Markdown>{article.content}</Markdown>
      </article>
    </div>
  );
};

export default ArticlePage;