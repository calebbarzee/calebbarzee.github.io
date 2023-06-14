export default function ProjectCard() {
   //template for project card that links to page about project
   return (
      <div className="bg-white font-semibold text-center rounded-3xl border shadow-lg p-10 max-w-xs">
         <img className="mb-3 w-32 h-32 rounded-full shadow-lg mx-auto" src="https://images.unsplash.com/photo-1633332755192-727a05c4013d?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=880&q=80" alt="product designer"></img>
         <h1 className="text-lg text-gray-700"> 8-bit-gallery </h1>
         <h3 className="text-sm text-gray-400 "> <b>Industry:</b> Education </h3>
         <p className="text-xs text-gray-400 mt-4"> Immerse yourself in a unique art history journey through this pixelated-react web app, where famous paintings are transformed into interactive 8-bit masterpieces, allowing you to both educate yourself and challenge your knowledge of renowned artworks. </p>
         <button className="bg-blue-500 px-8 py-2 mt-8 rounded-3xl text-gray-100 font-semibold uppercase tracking-wide">LEARN MORE</button>
      </div>
   );
}