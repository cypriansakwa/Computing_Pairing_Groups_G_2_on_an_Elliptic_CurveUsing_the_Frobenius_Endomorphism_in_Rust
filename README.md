# Computing Pairing Groups $G_2$ on an Elliptic CurveUsing the Frobenius Endomorphism in Rust

## Overview

This Rust program defines and operates on elements of the finite field $\mathbb{F}_{5^2}$ and implements elliptic curve operations over this field. The primary functionalities include:

- Field arithmetic (addition, subtraction, multiplication, inversion)
- Elliptic curve point operations (addition, doubling, scalar multiplication)
- Frobenius automorphism for characteristic 5 fields
- Finding full $r$-torsion points on the elliptic curve
- Identifying points in the subgroup $G_2$ satisfying $(x^5, y^5) = 5P$

## Features

### 1. **Finite Field $\mathbb{F}_{5^2}$**
- The field elements are represented as $a + bt$ where $t^2 = 3 \mod 5$.
- Basic arithmetic is implemented with modular reduction.

### 2. **Elliptic Curve Operations**
- The elliptic curve is defined as:
  $y^2 = x^3 + ax + b$
  over $\mathbb{F}_{5^2}$.
- Implements:
  - Point addition and doubling
  - Scalar multiplication using double-and-add method
  - Frobenius endomorphism
  - Full $r$-torsion point detection
  - Finding points in the subgroup $G_2$

## Usage

Compile and run using:

```sh
cargo run
