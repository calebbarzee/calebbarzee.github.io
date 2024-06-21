import Link from "next/link";

export default function Homepage() {

  return (
    <main className="flex min-h-screen flex-col items-center">
      <h3 className="text-3xl font-bold">You Made It!</h3>
    	<div className="mb-6 max-w-full items-center justify-between p-6">
			<p>
				Welcome to the home of coding fun for the summer!
				Here you can find all the links to resources we&apos;ll be using in our projects.
			</p>
    	</div>
    	<div>
			<ul>
				<li><Link href="https://codepen.io/calebbarzee/pen/VwOroVq">
					<h6 className="text-xl hover:underline">• Text_Adventure: Code_Quest -&gt;</h6>
				</Link></li>
				<li><Link href="https://codepen.io/calebbarzee/pen/ZENxqoK">
					<h6 className="text-xl hover:underline">• Personal_Webpage-&gt;</h6>
				</Link></li>
				<li><Link href="https://github.com/calebbarzee/kids_can_code">
					<h6 className="text-xl hover:underline">• Kids_Can_Code: All_Projects_Repository -&gt;</h6>
				</Link></li>
			</ul>
        </div>
			<h4 className="text-xl font-bold">Happy Coding!</h4>
			<img src="/typing_pixel_art.gif" alt="pixel art gif of typing on a keyboard" className="w-full h-auto" />
    </main>
  );
}
