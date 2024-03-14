
import AnimatePi from "../../components/AnimatePi.js";
import PiTrain from "../../components/PiTrain.js";
import getText from '../../utils/fetch_text.js';

export default function Homepage() {
	const PI = getText('app/utils/math/pi');

	  return (
		<main className="flex min-h-screen flex-col items-center">
		<h3 className="text-3xl font-bold">Happy Pi Day!</h3>
		<div className="p-6 mb-6 items-center justify-between max-w-full">
			<PiTrain PI={PI}/>
		</div>
		<AnimatePi PI={PI} />
    </main>
  );
}