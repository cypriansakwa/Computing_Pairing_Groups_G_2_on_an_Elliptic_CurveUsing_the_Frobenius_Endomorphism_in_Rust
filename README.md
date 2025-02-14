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
### Prerequisites

- Rust installed. If not, install it using [rustup](https://rustup.rs/).
- Cargo package manager (comes with Rust).

### Installation

Clone this repository:

```sh
git clone https://github.com/cypriansakwa/Computing_Pairing_Groups_G_2_on_an_Elliptic_CurveUsing_the_Frobenius_Endomorphism_in_Rust.git
cd Computing_Pairing_Groups_G_2_on_an_Elliptic_CurveUsing_the_Frobenius_Endomorphism_in_Rust
### Compiling and Running

Ensure you have Rust installed. Then, compile and run the program using:

```sh
cargo run
```
## Example Output

The program prints:
```
Elements of F(5^2):
0
1t
2t
3t
4t
1
1 + 1t
1 + 2t
1 + 3t
1 + 4t
2
2 + 1t
2 + 2t
2 + 3t
2 + 4t
3
3 + 1t
3 + 2t
3 + 3t
3 + 4t
4
4 + 1t
4 + 2t
4 + 3t
4 + 4t

Points on the elliptic curve y^2 = x^3 + x + 1:
(0, 1)
(0, 4)
(1, 1t)
(1, 4t)
(1 + 2t, 1 + 1t)
(1 + 2t, 4 + 4t)
(1 + 3t, 1 + 4t)
(1 + 3t, 4 + 1t)
(2, 1)
(2, 4)
(2 + 2t, 1t)
(2 + 2t, 4t)
(2 + 3t, 1t)
(2 + 3t, 4t)
(3, 1)
(3, 4)
(3 + 1t, 1 + 3t)
(3 + 1t, 4 + 2t)
(3 + 2t, 2)
(3 + 2t, 3)
(3 + 3t, 2)
(3 + 3t, 3)
(3 + 4t, 1 + 2t)
(3 + 4t, 4 + 3t)
(4, 2)
(4, 3)

Full 3-torsion points (3P = O):
(1, 1t)
(1, 4t)
(1 + 2t, 1 + 1t)
(1 + 2t, 4 + 4t)
(1 + 3t, 1 + 4t)
(1 + 3t, 4 + 1t)
(2, 1)
(2, 4)
Point at infinity

Group G₂ points (P such that (x^5, y^5) = 5P):
(1, 1t)
(1, 4t)
Point at infinity
```
## Implementation Details

### Field Element Representation (F5x2)  
The struct `F5x2` represents elements as `a + bt`.  

**Implements:**  
- Arithmetic operations (`add`, `sub`, `mul`, `div`)  
- Modular inversion  
- Frobenius automorphism:  
  $(a + bt)^5 = a + 4bt$

### Elliptic Curve Representation (Point)  
The struct `Point` represents points on the curve.  

**Supports:**  
- Point addition and doubling  
- Scalar multiplication  
- Frobenius transformation  
- Finding full `r`-torsion points and subgroup `G2`  

### Future Improvements  
- Extend support for general field characteristics.  
- Optimize scalar multiplication using windowed methods.  
- Explore alternative irreducible polynomials for field extensions.  

