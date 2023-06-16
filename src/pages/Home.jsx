import About from '../components/About';
import ProjectCard from '../components/subcomponents/ProjectCard';
import Education from '../components/Education';
import Skills from '../components/Skills';
import Work from '../components/Work';
import memoji0 from '../assets/memoji/waving.png';
import memoji1 from '../assets/memoji/smile.png'
import memoji2 from '../assets/memoji/lightbulb.png'
import memoji3 from '../assets/memoji/clouds.png'

import project1 from '../assets/project_images/8_bit_art.jpg'
// import project2 from '../assets/project_images/recycling_app.jpg'
// import project3 from '../assets/project_images/ai_assistant.jpg'

export default function Home() {
   return (
   <div className="grid gap-y-12 grid-cols-7">
      <div className="mt-12 col-start-2 col-end-6">
      <About />
      </div>
      <img src={memoji0} alt="Caleb waving memoji" className="w-44 h-auto rounded-full row-start-1 col-start-6 col-end-7 mt-8"></img>

         <div className="row-start-2 col-start-4 col-end-5 border-t border-blue-gray-50 py-1"></div>

      <div className="col-start-2 col-end-7">
      <Skills />
      </div>
      
      <div className="row-start-4 col-start-4 col-end-5 border-t border-blue-gray-50 py-1"></div>

      <div className="col-start-2 col-end-6">
      <Education />
      </div>

      <div className="row-start-6 col-start-4 col-end-5 border-t border-blue-gray-50 py-1"></div>

      <h3 className="text-xl font-bold row-start-6 col-start-2 col-end-6 text-left mt-4">Projects: </h3>
      <img src={memoji1} alt="Caleb smiling memoji" className="w-44 h-auto mt-8 row-start-7 col-start-2"/>
      <div className="col-start-3 col-end-7">
         <ProjectCard 
         title="8-bit-gallery"
         industry="Education"
         description="Immerse yourself in a unique art history journey through this pixelated-react web app, where famous paintings are transformed into interactive 8-bit masterpieces, allowing you to both educate yourself and challenge your knowledge of renowned artworks."
         image={project1}
         link="https://github.com/calebbarzee/8-bit-gallery"
         />
      </div>
      {/* <img src={memoji2} alt="Caleb lightbulb idea memoji" className="w-44 h-auto mt-8 col-start-6"/> */}
      {/* <div>
      <ProjectCard />
      </div>
      <img src={memoji3} alt="head in clouds memoji" className="w-44 h-auto mt-8 col-start-2"/>
      <div>
      <ProjectCard />
      </div> */}
      <div className="col-start-2 col-end-6 mb-8">
      <Work />
      </div>
   </div>
   );
}