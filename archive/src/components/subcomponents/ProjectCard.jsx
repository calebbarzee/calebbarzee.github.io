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
      <a href={props.link} className="block">
        <div className="p-4 bg-secondary dark:bg-dark-secondary shadow-custom-light dark:shadow-custom-dark rounded-lg text-left max-w-2xl self-center transition-colors duration-300 transform hover:bg-interactive hover:dark:bg-interactive-active">
          <img className="mb-3 rounded-md shadow-lg mx-auto" src={props.image} alt="Project Image" />
          <h1 className="text-lg font-semibold text-text-primary dark:text-dark-text-primary"> {props.title} </h1>
          <h3 className="text-sm text-text-secondary dark:text-dark-text-secondary"> <b>Industry:</b> {props.industry} </h3>
          <p className="text-xs mt-4 text-secondary dark:text-dark-text-secondary"> {props.description} </p>
        </div>
      </a>
    );    
}