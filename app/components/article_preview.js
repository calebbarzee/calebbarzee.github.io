import Link from "next/link";

const ArticlePreview = (props) => {
  return (
    <div className="rounded-md border-2 border-slate-300 p-4 shadow-sm">
      <p className="text-xs">{props.date_written}</p>

      <Link href={`/blog/${props.slug}`}>
        <h2 className="text-base hover:underline">{props.title}</h2>
      </Link>
      <p className="text-sm">{props.subtitle}</p>
    </div>
  );
};

export default ArticlePreview;
