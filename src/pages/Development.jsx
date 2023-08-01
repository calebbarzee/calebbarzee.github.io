import ProjectCard from '../components/subcomponents/ProjectCard';
import project1 from '../assets/project_images/8_bit_art.jpg';

export default function Development() {
   return (

      <div className="grid gap-y-12 grid-cols-7">
         <div className="col-start-3 col-end-7 mt-8 max-w-lg text-left">
        <h2 className="text-lg font-semibold">
          Software Development:
        </h2>
        <h4 className="text-md mt-2">Below are descriptions and links to some of my favorite projects that I&apos;ve completed. Feel free to explore and even analyze some of the code. I&apos;m all about writing clean, modular code.</h4>
      </div>
         <div className="col-start-2 col-end-7">
            <ProjectCard 
            title="8-bit-gallery"
            industry="Education"
            description="Immerse yourself in a unique art history journey through this pixelated-react web app, where famous paintings are transformed into interactive 8-bit masterpieces, allowing you to both educate yourself and challenge your knowledge of renowned artworks."
            image={project1}
            link="https://github.com/calebbarzee/8-bit-gallery"
            />
         </div>
         <div className="col-start-4 col-end-5 border-t border-blue-gray-50 py-1"></div>
         <div className="col-start-3 col-end-8">
            <ProjectCard 
            title="8-bit-gallery"
            industry="Education"
            description="Immerse yourself in a unique art history journey through this pixelated-react web app, where famous paintings are transformed into interactive 8-bit masterpieces, allowing you to both educate yourself and challenge your knowledge of renowned artworks."
            image={project1}
            link="https://github.com/calebbarzee/8-bit-gallery"
            />
         </div>
         <div className="col-start-4 col-end-5 border-t border-blue-gray-50 py-1"></div>
         <div className="col-start-2 col-end-7">
            <ProjectCard 
            title="8-bit-gallery"
            industry="Education"
            description="Immerse yourself in a unique art history journey through this pixelated-react web app, where famous paintings are transformed into interactive 8-bit masterpieces, allowing you to both educate yourself and challenge your knowledge of renowned artworks."
            image={project1}
            link="https://github.com/calebbarzee/8-bit-gallery"
            />
         </div>
         <div className="col-start-4 col-end-5 border-t border-blue-gray-50 py-1"></div>
         <h4 className="col-start-3 col-span-3 text-md mb-8">Hungry for more? Check out my github profile. If that proves insufficient, I&apos;m happy to answer inquiries via email as well!</h4>
      </div>
   );
}