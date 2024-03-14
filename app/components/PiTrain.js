'use client'
import React, { useState, useRef } from 'react';

		  /* <TypingArea
            completedText={completedText}
            inputText={inputText}
            errorIndex={errorIndex}
            duration={duration}
            started={started}
            incorrect={incorrect}
            inputRef={inputRef}
            handleKeyDown={handleKeyDown}
            remainingText={remainingText}
            handleStart={handleStart}
          /> */

		  
		//   function TypingArea({
		// 	started = false,
		// 	incorrect = false,
		// 	inputRef = null,
		// 	handleKeyDown,
		// 	remainingText = "",
		// 	completedText = "",
		// 	inputText = "",
		// 	errorIndex = 0,
		// 	handleStart
 


export default function PiTrain({PI}) {
  const [index, setIndex] = useState(0);
  const [digitCount, setDigitCount] = useState(0);
  const [errors, setErrors] = useState(0);
  const [percent, setPercent] = useState(100);
  const inputRef = useRef(null);
  const [isStarted, setIsStarted] = useState(false);
  const [isCorrect, setIsCorrect] = useState(true);
  const allowedKeys = "0123456789"

  const handleInput = e => {
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
	setPercent(Math.floor(((digitCount - errors) / digitCount) * 100))
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
	<p className="text-lg font-light">How many digits of π can you memorize?</p>
	<hr className="-mt-4 mb-5"></hr>
        <div className="grid grid-cols-3 gap-4">
			<p className="col-span-3">
            Digit Count: {digitCount} Errors: {errors} Accuracy: {percent}%
			</p>
        </div>
	  <div className='typing-box'>
				  <p className="text-xs -my-1">Starting with the first decimal place, type the digits of π as they appear below:</p>
				  <div
					className={`typing-area${isStarted ? "border-2 border-green-700" : ""}${isCorrect ? "" : "border-2 border-red-700"} border-2 border-slate-300 p-2 rounded-md`}
					ref={inputRef}
					aria-label='text-box'
					onKeyDown={handleInput}
					tabIndex={0}
					role='textbox'
				  >
					<span className='cursor' />
					<span className='remaining-text'>{PI.slice(index, index+30)}</span>
				  </div>
				</div>
				<div className='grid grid-cols-5 mt-4'>
				  <button
					type='button'
					className='hover:underline grid-start-2'
					disabled={isStarted}
					onClick={isStarted ? undefined : handleStart}
				  >
					Start
				  </button>
				<button className='hover:underline grid-start-3'onClick={handleReset}>Reset</button>
				</div>
      {/* <div>
        <input
          ref={inputRef}
          type="text"
		  placeholder={"3." + PI}
          onChange={handleInput}
          className="bg-transparent border-2 border-slate-300 p-2 rounded-md md:row-start-1 md:col-start-3"
        />
      </div> */}
    </>
  );
}
