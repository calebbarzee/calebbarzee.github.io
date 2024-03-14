'use client'
import React, { useState, useEffect } from 'react';

function PiComponent() {
	const generateDigitsOfPi = (isRunning, digits) => {
		let q = 1n;
		let r = 180n;
		let t = 60n;
		let i = 2n;
		while (isRunning && digits.length < 10) {
			let digit = ((i * 27n - 12n) * q + r * 5n) / (t * 5n);
			digits += digit;
			let u = i * 3n;
			u = (u + 1n) * 3n * (u + 2n);
			r = u * 10n * (q * (i * 5n - 2n) + r - t * digit);
			q *= 10n * i * (i++ * 2n - 1n);
			t *= u;
		}
		return digits;
	};

	const [digits, setDigits] = useState("");
	const [isRunning, setRunning] = useState(false);

	useEffect(() => {
		let interval;
		if (isRunning) {
			interval = setInterval(() => {
				const nextDigit = generateDigitsOfPi();
				setDigits(prevDigits => prevDigits + nextDigit);
			}, 100);
		}
		return () => clearInterval(interval);
	}, [isRunning]);

	const handleClick = () => {
		setRunning(prevRunning => !prevRunning);
	};

	return (
		<div>
			<button onClick={handleClick}>{isRunning ? "Stop" : "Start"}</button>
			<div className="break-words text-wrap">
				<p>{digits}</p>
			</div>
		</div>
	);
}

export default PiComponent;
