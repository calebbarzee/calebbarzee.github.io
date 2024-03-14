import AnimatePi from "../components/AnimatePi.js";
import getText from '../utils/fetch_text.js';

export default function Homepage() {
	const PI = getText('app/utils/math/pi');

	  return (
		<main className="flex min-h-screen flex-col items-center">
		<AnimatePi PI={PI} />
    </main>
  );
}