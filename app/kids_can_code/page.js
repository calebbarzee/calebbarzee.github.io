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
			{/* links go here */}
        </div>
			<h4 className="text-xl font-bold">Happy Coding!</h4>
			<img src="/typing_pixel_art.gif" alt="pixel art gif of typing on a keyboard" className="w-full h-auto" />
    </main>
  );
}
