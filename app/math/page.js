import PiComponent from "../components/CalcPi";

export default function Homepage() {

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="font-monospace">
        <h1 className="text-3xl">Happy Pi Day!!</h1>
        <PiComponent />
      </div>
    </main>
  );
}
