import Link from "next/link.js";
import AnimatePi from "../../components/AnimatePi.js";
import getText from '../../utils/fetch_text.js';

export default function Homepage() {
	const PI = getText('app/utils/math/pi');

	  return (
		<main className="flex min-h-screen flex-col items-center">
		<h3 className="text-3xl font-bold">Happy Pi Day!</h3>
		<Link href="/blog/pi_calc_blog1"><h6 className="hover:underline">Calculating the digits of π -&gt;</h6></Link>
		<AnimatePi PI={PI} />
    </main>
  );
}