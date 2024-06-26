import { Fira_Code } from "next/font/google";
import "./globals.css";
import Link from "next/link";

const fira = Fira_Code({ subsets: ["latin"] });

export const metadata = {
  title: "Caleb's World",
  description: "Caleb Barzee's Personal Website",
};

export default function RootLayout({ children }) {
  const header = (
    <header>
      <div className="mx-auto mt-20 grid max-w-full grid-cols-5 items-center justify-between gap-2">
        <div className="col-start-2">
          <Link href="/">
            <h6 className="hover:underline">Home -&gt;</h6>
          </Link>
        </div>
        <div className="col-start-3">
          <Link href="/blog">
            <h6 className="hover:underline">Blog -&gt;</h6>
          </Link>
        </div>
        <div className="col-start-4">
          <Link href="/kids_can_code/">
            <h6 className="hover:underline">Kids_Can_Code -&gt;</h6>
          </Link>
        </div>
      </div>
    </header>
  );

  const footer = <footer></footer>;

  return (
    <html lang="en">
      <head />
      <body className={fira.className}>
        <div className="mx-auto  max-w-6xl px-6">
          {header}
          {children}
          {footer}
        </div>
      </body>
    </html>
  );
}
