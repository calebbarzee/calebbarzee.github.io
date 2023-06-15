import PropTypes from 'prop-types';

export default function ProjectCard(props) {
   //template for project card that links to page about project

   ProjectCard.propTypes = {
      title: PropTypes.string.isRequired,
      industry: PropTypes.string.isRequired,
      description: PropTypes.string.isRequired,
      link: PropTypes.string.isRequired,
      image: PropTypes.any.isRequired,
      // image is the filename of the image and must be located in assets/projectImages
   };
   //props contains title, industry, description, image, link
   return (
      <a href={props.link}>
      <div className="p-4 bg-white shadow-md rounded-lg text-left max-w-2xl self-center">
         <img className="mb-3 rounded-md shadow-lg mx-auto" src={props.image} alt="Project Image"></img>
         <h1 className="text-lg font-semibold text-gray-700"> {props.title} </h1>
         <h3 className="text-sm text-gray-400 "> <b>Industry:</b> {props.industry} </h3>
         <p className="text-xs mt-4"> {props.description} </p>
      </div>
      </a>
   );
}