'use client'
import React, { useState, useEffect } from 'react';

export default function AnimatePi({PI}) {
	const [isRunning, setIsRunning] = useState(false);
	const [numOfDigits , setNumOfDigits] = useState(100);
	const [displayedDigits, setDisplayedDigits] = useState('3.');

	useEffect(() => {
		let interval;
		if (isRunning && numOfDigits > displayedDigits.length - 2) {
		  interval = setInterval(() => {
			setDisplayedDigits((prevDigits) => {
			  if (prevDigits.length - 2 < PI.length && prevDigits.length - 2 < numOfDigits) {
				return `3.${PI.slice(0, prevDigits.length - 1)}`;
			  } else {
				clearInterval(interval);
				return prevDigits;
			  }
			});
		  }, 30); // Update interval in milliseconds
		}
	
		return () => clearInterval(interval); // Cleanup on component unmount or isRunning change
	  }, [isRunning]); // Depend on isRunning to re-activate or deactivate the interval
	
	  return (
		<div className="flex flex-col gap-4 p-6 md:grid md:grid-cols-5 md:grid-rows-5 md:gap-4 min-h-3 items-center justify-between max-w-full">
		  <label className="md:row-start-1 md:col-start-1 md:col-span-2">Number of Digits: </label>
		  <input className="bg-transparent border-2 border-slate-300 p-2 rounded-md md:row-start-1 md:col-start-3" title="Iterations" type="number" value={numOfDigits} onChange={(e) => {setNumOfDigits(e.target.value); setIsRunning(false)}} />
		  <button className="self-center md:self-auto hover:underline md:row-start-1" onClick={() => setIsRunning(!isRunning)}>{isRunning ? "Stop" : "Start"}</button>
		  <div className="border-2 border-slate-300 p-4 rounded-md md:row-start-2 md:col-span-5 w-screen">
			<span className="text-2xl font-mono font-semibold break-words text-wrap">
			  {displayedDigits}
			</span>
		  </div>
		</div>
  );
}