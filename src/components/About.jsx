import memoji2 from '../assets/memoji/lightbulb.png';

export default function About() {
   // Elevator pitch of myself and career
   return (
   <section id="about" className="container mx-auto px-10 py-20 items-center">
        <div className="grid grid-cols-3 text-left">
          <img src={memoji2} alt="Caleb Profile Memoji Image" className="w-44 h-auto row-start-1 col-start-3 col-end-4"></img>
          <div className="max-w-lg row-start-1 col-start-1 col-end-3">
            <h1 className="title-font sm:text-4xl text-3xl mb-3 font-medium">
              Hi, I&apos;m Caleb.
            </h1>
            <h2 className="leading-relaxed">
              A collaborative and innovative professional, with a focus on human-centric design. You can catch my everyday creating or cyclying.
              <br></br><b className="text-lg">Biking boosts my bytes!</b>
            </h2>
          </div>
      </div>
    </section>
   );
}