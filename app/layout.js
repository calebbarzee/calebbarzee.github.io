import { Fira_Code } from "next/font/google";
import "./globals.css";

const fira = Fira_Code({ subsets: ["latin"] });

export const metadata = {
  title: "Caleb's World",
  description: "Caleb Barzee's Personal Website",
};

export default function RootLayout({ children }) {
  const header = (
    <header>
    </header>
  );

  const footer = (
    <footer>
    </footer>
  );

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
