import About from "../components/About";
import ProjectCard from "../components/ProjectCard";
import Education from "../components/Education";
import Skills from "../components/Skills";
import Work from "../components/Work";

export default function Home() {
   return (
   <div className="homepage">
      <About />
      {/* js here to render out three project card templates */}
      <ProjectCard />
      <Education />
      <Skills />
      <Work />
   </div>
   )
}