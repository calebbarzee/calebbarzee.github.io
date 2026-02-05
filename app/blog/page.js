import { get_metadata } from "/app/utils/get_article";
import ArticlePreview from "/app/components/article_preview";

const HomePage = () => {
  const article_metadata = get_metadata();
  const ArticlePreviews = article_metadata.map((article) => (
    <ArticlePreview key={article.slug} {...article} />
  ));

  return <div className="mx-auto my-4 max-w-3xl">{ArticlePreviews}</div>;
};

export default HomePage;
