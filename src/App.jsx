import React from "react";
import Navbar from "./components/Navbar";
import Footer from "./components/Footer";
import Home from "./pages/Home";
import './App.css'

export default function App() {
  return (
    <main>
      <Navbar />
        <Home />
      <Footer />
    </main>
  );
}
