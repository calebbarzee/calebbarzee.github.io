import { Link } from 'react-router-dom';

export default function Navbar() {
    return (
    <nav className="px-2 shadow-custom-light dark:shadow-custom-dark rounded-lg text-left max-w-2xl self-center mx-auto">
     <div className="container flex items-center justify-center p-6 mx-auto text-secondary capitalize dark:text-dark-text-secondary">
         <Link to="/" className="text-text-primary transition-colors duration-300 transform dark:text-dark-text-primary border-b-2 hover:border-interactive mx-1.5 sm:mx-6">home</Link>
 
         <Link to="/design" className="border-b-2 border-transparent hover:text-text-primary transition-colors duration-300 transform dark:hover:text-dark-text-primary hover:border-interactive mx-1.5 sm:mx-6">design</Link>
 
         <Link to="/development" className="border-b-2 border-transparent hover:text-text-primary transition-colors duration-300 transform dark:hover:text-dark-text-primary hover:border-interactive mx-1.5 sm:mx-6">development</Link>
 
         <Link to="/resume" className="border-b-2 border-transparent hover:text-text-primary transition-colors duration-300 transform dark:hover:text-dark-text-primary hover:border-interactive mx-1.5 sm:mx-6">resume</Link>
     </div>
    </nav>
    );
 }