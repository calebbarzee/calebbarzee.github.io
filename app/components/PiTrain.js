"use client";
import React, { useState, useRef } from "react";

export default function PiTrain({ PI }) {
  const [index, setIndex] = useState(0);
  const [digitCount, setDigitCount] = useState(0);
  const [errors, setErrors] = useState(0);
  const [percent, setPercent] = useState(100);
  const inputRef = useRef(null);
  const [isStarted, setIsStarted] = useState(false);
  const [isCorrect, setIsCorrect] = useState(true);
  const allowedKeys = "0123456789";

  const handleInput = (e) => {
    e.preventDefault();
    const { key } = e;
    if (allowedKeys.includes(key) && key === PI.charAt(index)) {
      setIsCorrect(true);
      setDigitCount(digitCount + 1);
      setIndex(index + 1);
    } else {
      setIsCorrect(false);
      setErrors(errors + 1);
    }
    setPercent(Math.floor(((digitCount - errors) / digitCount) * 100));
  };

  const handleReset = () => {
    setIndex(0);
    setErrors(0);
    setDigitCount(0);
    setPercent(100);
    setIsStarted(false);
    setIsCorrect(true);
  };

  const handleStart = () => {
    setIsStarted(true);
    inputRef.current.focus();
  };

  return (
    <>
      <h5 className="text-3xl font-bold">π Training Grounds:</h5>
      <p className="text-lg font-light">
        How many digits of π can you memorize?
      </p>
      <hr className="-mt-4 mb-5 w-full"></hr>
      <div className="grid grid-cols-3 gap-4">
        <p className="col-span-3">
          Digit Count: {digitCount} Errors: {errors} Accuracy: {percent}%
        </p>
      </div>
      <div className="typing-box">
        <p className="-my-1 text-xs">
          Starting with the first decimal place, type the digits of π as they
          appear below:
        </p>
        <input
          className={`${isStarted ? "border-2 focus:border-green-700" : ""}${isCorrect ? "" : "border-2 focus:border-red-700"} rounded-md border-2 border-slate-300 p-2 bg-transparent focus:ring-0 focus:outline-none`}
          ref={inputRef}
          aria-label="text-box"
          onKeyDown={handleInput}
          tabIndex={0}
          role="textbox"
          rows={1}
          cols={30}
          value={PI.slice(index, index + 30)}
          readOnly
        />
      </div>
      <div className="mt-4 grid grid-cols-5">
        <button
          type="button"
          className="grid-start-2 hover:underline"
          disabled={isStarted}
          onClick={isStarted ? undefined : handleStart}
        >
          Start
        </button>
        <button className="grid-start-3 hover:underline" onClick={handleReset}>
          Reset
        </button>
      </div>
    </>
  );
}
