"use client";
import React, { useState, useEffect } from "react";
import Link from "next/link.js";

export default function AnimatePi({ PI }) {
  const [isRunning, setIsRunning] = useState(false);
  const [numOfDigits, setNumOfDigits] = useState(100);
  const [displayedDigits, setDisplayedDigits] = useState("3.");

  useEffect(() => {
    let interval;
    if (isRunning && numOfDigits > displayedDigits.length - 2) {
      interval = setInterval(() => {
        setDisplayedDigits((prevDigits) => {
          if (
            prevDigits.length - 2 < PI.length &&
            prevDigits.length - 2 < numOfDigits
          ) {
            return `3.${PI.slice(0, prevDigits.length - 1)}`;
          } else {
            clearInterval(interval);
            return prevDigits;
          }
        });
      }, 30); // Update interval in milliseconds
    }

    return () => clearInterval(interval); // Cleanup on component unmount or isRunning change
  }, [isRunning, PI, displayedDigits, numOfDigits]); // Depend on isRunning to re-activate or deactivate the interval

  return (
    <>
      <h5 className="mb-6 text-2xl font-semibold">
        Discovering the Digits of Pi
      </h5>
      <hr className="-mt-6 mb-8 w-full"></hr>
      <Link href="/blog/pi_calc_blog1">
        <h6 className="hover:underline">Calculating the digits of π -&gt;</h6>
      </Link>
      <Link href="https://en.wikipedia.org/wiki/Approximations_of_π#">
        <h6 className="hover:underline">Approximations of π (Wiki) -&gt;</h6>
      </Link>
      <div className="flex min-h-3 max-w-full flex-col items-center justify-between gap-4 p-6 md:grid md:grid-cols-5 md:grid-rows-5 md:gap-4">
        <label className="md:col-span-2 md:col-start-1 md:row-start-1">
          Number of Digits:{" "}
        </label>
        <input
          className="rounded-md border-2 border-slate-300 bg-transparent p-2 md:col-start-3 md:row-start-1"
          title="Iterations"
          type="number"
          value={numOfDigits}
          onChange={(e) => {
            setNumOfDigits(e.target.value);
            setIsRunning(false);
            setDisplayedDigits("3.");
          }}
        />
        <button
          className="self-center hover:underline md:row-start-1 md:self-auto"
          onClick={() => setIsRunning(!isRunning)}
        >
          {isRunning ? "Stop" : "Start"}
        </button>
        <div className="w-full rounded-md border-2 border-slate-300 p-4 md:col-span-5 md:row-start-2">
          <span className="text-wrap break-words font-mono text-2xl font-semibold">
            {displayedDigits}
          </span>
        </div>
      </div>
    </>
  );
}
