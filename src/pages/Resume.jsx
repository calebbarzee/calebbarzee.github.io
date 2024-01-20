import PDFPreview from '../components/utilities/PDFPreview';

export default function Resume() {
   return (
      <div className="grid gap-y-12 grid-cols-7">
         <h2 className="text-xl font-bold mt-12 col-start-2 col-end-3 text-text-primary dark:text-dark-text-primary">Resume:</h2>
         <div className="bg-secondary dark:bg-dark-secondary shadow-custom-light dark:shadow-custom-dark rounded-lg text-left max-w-2xl self-center col-start-1 col-end-8 place-self-center">
         <PDFPreview />
         </div>
         <button className="col-span-1 col-start-4 place-self-center p-4 mb-12 bg-secondary dark:bg-dark-secondary shadow-custom-light dark:shadow-custom-dark rounded-lg text-center">
            <a href="documents/resume.pdf" download className="text-text-primary dark:text-dark-text-primary border-b-2 border-transparent hover:border-interactive-active transition-colors duration-300 transform">
               Download
            </a>
         </button>
      </div>
   );
}
