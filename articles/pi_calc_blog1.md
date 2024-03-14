---
title: "Calculating Pi"
subtitle: "An engaging algorithmic topic."
date_written: "2023-10-23"
---

## Why?

The quest to calculate the value of pi (π) has fascinated mathematicians for centuries. The number π, which represents the ratio of a circle's circumference to its diameter, is an irrational number that continues infinitely without repetition or pattern. Over the years, various formulas have been devised to calculate π, ranging from simple approximations to complex series that require advanced mathematical techniques for their derivation. In this post, we'll explore three formulas for π, moving from relatively simple to more complex.

## 1. The Leibniz Formula for π

The Leibniz formula for π is a beautifully simple yet infinitely complex series. It is given by:

### _π/4 = 1 - 1/3 + 1/5 - 1/7 + 1/9 - ..._

This formula falls under the "easy" category because of its straightforward pattern. However, it converges very slowly to π, which means that a large number of terms are required to get an accurate value of π.

## 2. Machin's Formula

John Machin, in 1706, devised a formula that significantly improved the speed of π calculations:

### _π/4 = 4 _ arctan(1/5) - arctan(1/239)\*

This formula uses the inverse tangent function and is more efficient than the Leibniz series. Machin's approach allowed for quicker computation of π with fewer terms, marking it as a medium difficulty in our list.

## 3. Chudnovsky Algorithm

For those craving complexity, the Chudnovsky algorithm provides a giant leap in terms of both sophistication and efficiency. The formula is given by:

### _π = 1/12 _ ∑ (from k=0 to ∞) of ((-1)^k _ (6k)! _ (545140134k + 13591409)) / ((3k)! _ (k!)^3 _ (640320)^(3k + 3/2))\*

This formula, derived by the Chudnovsky brothers in the 20th century, has been used to calculate billions of digits of π and represents the "complex" category due to its use of factorials, powers, and the astonishing rate of convergence.

## Conclusion

The journey from the simple alternating series of Leibniz to the rapid convergence of the Chudnovsky algorithm illustrates the depth and beauty of mathematical exploration into π. Each formula not only provides a method for calculating π but also offers insights into the nature of mathematical series, convergence, and the limitless horizons of mathematical discovery.
