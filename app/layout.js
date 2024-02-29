import { Fira_Code } from "next/font/google";
import "./globals.css";

const fira = Fira_Code({ subsets: ["latin"] });

export const metadata = {
  title: "Caleb's World",
  description: "Caleb Barzee's Personal Website",
};

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <body className={fira.className}>{children}</body>
    </html>
  );
}
