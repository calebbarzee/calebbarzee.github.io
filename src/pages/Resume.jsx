import PDFPreview from '../components/utilities/PDFPreview';

export default function Resume() {
   return (
      <div className="grid gap-y-12 grid-cols-7">
         <h2 className="text-xl font-bold mt-12 col-start-2 col-end-3">Resume: </h2>
         <div className="p-4 bg-white shadow-md rounded-lg text-left max-w-2xl self-center col-start-1 col-end-8 place-self-center">
         <PDFPreview />
         </div>
         <button className="col-span-1 col-start-4 place-self-center p-4 mb-12 bg-white shadow-md rounded-lg text-center "><a href="documents/resume.pdf" download className="border-b-2 transition-colors duration-300 transform hover:border-blue-500">Download</a></button>
      </div>
   );
}