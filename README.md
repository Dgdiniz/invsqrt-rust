# Inverse Square Root in Rust

This repository contains a simple implementation of the fast inverse square root algorithm in Rust. The project demonstrates how to perform the computation of the inverse square root of a given float number.

## Usage

To use this project, you can call the `invsqrt()` function with a float value as an argument, and it will return the inverse square root of the given number.

```rust
fn main() {
    println!("{}", invsqrt(16.0));
}
```

## Algorithm
The `invsqrt()` function uses the famous fast inverse square root algorithm, which makes use of bit manipulation to quickly approximate the inverse square root of a given float number.

```rust
fn invsqrt(x: f32) -> f32 {
    let xhalf: f32 = 0.5 * x;
    let mut i = x.to_bits();

    i = 0x5f3759df - (i >> 1);

    let mut y = f32::from_bits(i);
    y = y * (1.5f32 - xhalf * y * y);

    return y;
}
```

## In-Depth Explanation of the Algorithm

The fast inverse square root algorithm is an approximation method that computes the inverse square root of a single-precision floating-point number. The algorithm was popularized by its use in the 3D video game _Quake_ by id Software, though its origins date back to the early 1990s.

### Algorithm Steps

1. Multiply the input value `x` by 0.5 and store the result in a variable `xhalf`.
2. Convert the input value `x` to its 32-bit integer representation using the `to_bits()` method.
3. Perform a right bitwise shift of the integer representation by 1.
4. Subtract the shifted integer representation from the magic number `0x5f3759df`. This magic number is derived from the IEEE 754 floating-point standard and is the key to the algorithm's speed and accuracy.
5. Convert the result back to a floating-point number using the `from_bits()` method.
6. Perform one iteration of the Newton-Raphson method to refine the approximation. Multiply the current approximation `y` by `(1.5 - xhalf * y * y)`.
7. Return the final approximation of the inverse square root.

### How it Works

The fast inverse square root algorithm exploits the properties of IEEE 754 floating-point representation and the relationship between the exponent and the square root function. The key insight behind the algorithm is that the subtraction of the magic number `0x5f3759df` is an approximation of the inverse square root for a wide range of input values.

The Newton-Raphson step refines the initial approximation, providing a more accurate result. In most cases, a single iteration is sufficient, but additional iterations can be used for higher precision at the cost of computational speed.

It is important to note that this algorithm is an approximation method and may not provide the same level of accuracy as other methods, such as those provided by math libraries. However, its speed and simplicity make it suitable for applications where performance is a priority and minor inaccuracies are acceptable, such as in video game engines.

## References

- Lomont, C. (2003). Fast Inverse Square Root. Retrieved from http://www.lomont.org/Math/Papers/2003/InvSqrt.pdf
- Eberly, D. (2002). Fast Inverse Square Root. Geometric Tools. Retrieved from https://www.geometrictools.com/Documentation/FastInverseSqrt.pdf
