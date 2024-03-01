import {get_metadata} from "../utils/get_article";
import ArticlePreview from "../components/ArticlePreview";

const HomePage = () => {
  const article_metadata = get_metadata();
  const ArticlePreviews = article_metadata.map((article) => (
    <ArticlePreview key={article.slug} {...article} />
  ));

  return (
    <div className="my-4 mx-auto max-w-3xl">{ArticlePreviews}</div>
  );
};

export default HomePage;