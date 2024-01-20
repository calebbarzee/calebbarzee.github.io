import { Routes, Route } from 'react-router-dom';
import Layout from './pages/Layout';
import Home from './pages/Home';
import Design from './pages/Design';
import Development from './pages/Development';
import Resume from './pages/Resume';
import NoPage from './pages/NoPage';

export default function App() {
  return (
    <main className="max-w-1280 mx-auto p-8 text-center bg-primary dark:bg-dark-primary text-text-primary dark:text-dark-text-primary">
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route path="" element={<Home />} />
          <Route path="design" element={<Design />} />
          <Route path="development" element={<Development />} />
          <Route path="resume" element={<Resume />} />
          <Route path="*" element={<NoPage />} />
        </Route>
      </Routes>
    </main>
  );
}
