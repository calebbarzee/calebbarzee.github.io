import AnimatePi from "../../components/AnimatePi.js";
import PiTrain from "../../components/PiTrain.js";
import getText from "../../utils/fetch_text.js";

export default function Homepage() {
  const PI = getText("app/utils/math/pi");

  return (
    <main className="flex min-h-screen flex-col items-center">
      <h3 className="text-3xl font-bold">Happy Pi Day!</h3>
      <div className="mb-6 max-w-full items-center justify-between p-6">
        <div>
          <PiTrain PI={PI} />
        </div>
        <div className="mt-10">
          <AnimatePi PI={PI} />
        </div>
      </div>
    </main>
  );
}
