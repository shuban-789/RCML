# rcl

## About

Rudimentary Calculus Library, or `rcl`, is a simple calculus package which aims to provide both single variable and multivariable calculus functions for programs written in Rust or C/C++

Limits and any limit dependent function will not be perfectly correct but extremely close to the correct value due to the way they are calculated. A standard convention for tolerance may be used in the future. Integration follows Simpson's Rule calculation as of now.

## TODO
- Potentially make an `nderive()` function such that any nth derivative can be calculated with minimal hard coding
- Potentially figure out a system for working with returning indefinite results for integration or differentiation 
