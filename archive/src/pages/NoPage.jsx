export default function NoPage() {
   const goBack = () => {window.history.back();};
   return (
         <div className="container flex justify-center items-center min-h-screen px-6 py-12 mx-auto">
            <div>
                  <p className="text-sm font-medium text-interactive dark:text-interactive">404 error</p>
                  <h1 className="mt-3 text-2xl font-semibold text-text-primary dark:text-dark-text-primary md:text-3xl">We can't find that page</h1>
                  <p className="mt-4 text-secondary dark:text-dark-text-secondary">Sorry, the page you are looking for doesn't exist or has been moved.</p>

                  <div className="flex items-center mt-6 gap-x-3">
                     <button onClick={goBack} className="flex items-center justify-center w-1/2 px-5 py-2 text-sm text-secondary transition-colors duration-200 bg-white border rounded-lg gap-x-2 sm:w-auto dark:bg-dark-primary dark:hover:bg-dark-secondary hover:bg-primary dark:text-dark-text-secondary dark:border-dark-quaternary">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth="1.5" stroke="currentColor" className="w-5 h-5 rtl:rotate-180">
                              <path strokeLinecap="round" strokeLinejoin="round" d="M6.75 15.75L3 12m0 0l3.75-3.75M3 12h18" />
                        </svg>

                        <span>Go back</span>
                     </button>
                  </div>
            </div>
         </div>
   );
}
