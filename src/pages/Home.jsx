import About from '../components/About';
// import ProjectCard from '../components/subcomponents/ProjectCard';
import Education from '../components/Education';
// import Skills from '../components/Skills';
// import Work from '../components/Work';
import memoji3 from '../assets/memoji/clouds.png'

export default function Home() {
   return (
   <div className="grid grid-cols-7">
      <div className="col-span-full mx-auto">
      <About />
      </div>
      <div className="col-span-full mb-8 mx-auto">
      <Education />
      </div>
      <img src={memoji3} alt="head in clouds memoji" className="w-44 h-auto mt-8 col-start-2"/>
      {/* js here to render out three project card templates */}
      {/* <ProjectCard />
      <Work /> 
      <Skills /> */}
   </div>
   );
}