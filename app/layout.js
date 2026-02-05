import { Fira_Code } from "next/font/google";
import NavBar from "/app/components/NavBar";
import "./globals.css";

const fira = Fira_Code({ subsets: ["latin"] });


export default function RootLayout({ children }) {


  const footer = <footer></footer>;

  return (
    <html lang="en">
      <head />
      <body className={fira.className}>
        <div className="mx-auto  max-w-6xl px-6">
          <NavBar />
          {children}
          {footer}
        </div>
      </body>
    </html>
  );
}
