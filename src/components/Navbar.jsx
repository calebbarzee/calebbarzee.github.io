import { Link } from 'react-router-dom';
// import Home from "../../pages/Home";
// import Design from "../../pages/Design";
// import Development from "../../pages/Development";
// import Resume from "../../pages/Resume";

export default function Navbar() {
   //navbar included on every page
   return (
   <nav className="px-2 bg-white shadow-md rounded-lg text-left max-w-2xl self-center mx-auto">
    <div className="container flex items-center justify-center p-6 mx-auto text-gray-600 capitalize dark:text-gray-300">
        <Link to="/home" className="text-gray-800 transition-colors duration-300 transform dark:text-gray-200 border-b-2 hover:border-blue-500 mx-1.5 sm:mx-6">home</Link>

        <Link to="/design" className="border-b-2 border-transparent hover:text-gray-800 transition-colors duration-300 transform dark:hover:text-gray-200 hover:border-blue-500 mx-1.5 sm:mx-6">design</Link>

        <Link to="/development" className="border-b-2 border-transparent hover:text-gray-800 transition-colors duration-300 transform dark:hover:text-gray-200 hover:border-blue-500 mx-1.5 sm:mx-6">development</Link>

        <Link to="/resume" className="border-b-2 border-transparent hover:text-gray-800 transition-colors duration-300 transform dark:hover:text-gray-200 hover:border-blue-500 mx-1.5 sm:mx-6">resume</Link>
    </div>
   </nav>
   );
}