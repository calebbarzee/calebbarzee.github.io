// contains the always present elements like footer, navbar, ect.
import { Outlet } from 'react-router-dom';
import Navbar from '../components/Navbar';
import Footer from '../components/Footer';

export default function Layout() {
   return(
      <div className="pageContainer">
         <Navbar />
         <Outlet />
         <Footer />
      </div>
   );
}