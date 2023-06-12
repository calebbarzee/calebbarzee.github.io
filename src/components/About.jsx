import memoji2 from "../assets/memoji/caleb_memoji2.jpeg";

export default function About() {
   // Elevator pitch of myself and career
   return (
   <section id="about">
      <div className="container mx-auto flex px-10 py-20 md:flex-row flex-col items-center">
        <div className="lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center">
          <h1 className="title-font sm:text-4xl text-3xl mb-4 font-medium">
            Hi, I&#39;m Caleb.
          </h1>
          <img src={memoji2} alt="Caleb Profile Memoji Image" className="w-40 h-40 rounded-full"></img>
          <h2 className="mb-8 leading-relaxed">
            A collaborative and innovative professional, with a focus on human-centric design.You can catch my everyday creating or cyclying.
            Biking boosts my bytes!
          </h2>
        </div>
      </div>
    </section>
   )
}