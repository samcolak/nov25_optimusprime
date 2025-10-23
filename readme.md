# OptimusPrime - November 2025 Challenge

Determine the highest prime number within 30 seconds

## What is a Prime Number

A prime number is a number that can only be divisable by itself and one.

### Side notes

1. 0 is not a prime number
2. All even numbers (except 2) are not prime numbers
3. The maximum number of steps to determine a factor is 1/2 of the number being resolved
4. Anything divisable by 5 is also not a prime (as it is odd, this is checked by step 2)

### Extras

1. A prime is known to exhibit a property where k = (6n + 1) or k = (6n - 1) however using this is not alone sufficient
2. Testing factors should use the prime set as opposed to even numbers - All numbers that are exclusive to this set themselves are already known to be factored.
3. The max testing range for a factor does not need to exceed the sqrt(k). Anything further is superfluous

## Challenge

Part of the Rust developers challenge

