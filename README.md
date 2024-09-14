# A simple Rust powered Command Line Calculator

## Currently available Operators and Functions
* Addition `+`
* Subtraction `-`
* Multiplication `-`
* Division `/`
* Exponents `p`
* Sine `sin()`
* Cosine `cos()`
* Tangent `tan()`

## Technology

This calculator uses the Shunting Yard Algorithm and utilizes Zero Sized Types (ZSTs) for it´s operators.
Operators implement a common Trait, that lets them define their symbol, operation, precedence and the number of required operands.
Because of this, new operators may be added with relativ ease.

## Arguments

* `-p` Skip rounding the result to two decimal points  
