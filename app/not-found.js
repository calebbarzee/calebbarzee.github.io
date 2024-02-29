export default function notFound() {
  return (
	<main className="flex min-h-screen flex-col items-center justify-between p-24">
		<div classname="font-monospace">
			<h1 className="text-lg">404 - Page Not Found</h1>
			<p>Sorry, the page you are looking for does not exist.</p>
			<a href="/">Go back home -&gt;</a>
		</div>
	</main>
  )
}
