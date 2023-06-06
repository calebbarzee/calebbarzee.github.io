import React from "react";
import About from "./components/About";
import Contact from "./components/Contact";
import Navbar from "./components/Navbar";
import Projects from "./components/Projects";
import Skills from "./components/Skills";
import Work from "./components/Work";
import Footer from "./components/Footer";
import './App.css'

export default function App() {
  return (
    <main>
      <Navbar />
      <About />
      <Projects />
      <Work />
      <Skills />
      <Contact />
      <Footer />
    </main>
  );
}