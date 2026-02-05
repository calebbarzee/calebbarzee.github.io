"use client"

import Link from "next/link";
import { usePathname } from "next/navigation";

export default function NavBar() {
  const path = usePathname()
  const navLinks = [
    {
      href: path === "/" ? "/blog" : "/",
      text: path === "/" ? "Blog" : "Home",
    },
  ];

console.log("navLinks:", navLinks)
return (
    <header>
      <div className="mx-auto mt-20 grid max-w-full grid-cols-5 items-center justify-between gap-2">
        {navLinks.map((link, index) => (
          <div key={link.text} className={`col-start-${index + 2}`}>
            <Link href={link.href}>
              <h6 className="hover:underline">{link.text} -&gt;</h6>
            </Link>
          </div>
        ))}
      </div>
    </header>
  )
}
