# Complex Numbers & Functions

- Supports Complex Numbers and Functions relating to Complex Numbers

***

### Complex Numbers:

Complex Numbers are numbers that have both a real and imaginary part `i`. They are represented as `a + bi` where `a` is the real part and `b` is the imaginary part. `i` is the imaginary unit, where `i^2 = -1`.

Complex Numbers are supported by the struct `Complex` in the `numerlib::Complex` module. To create a new Complex Number, use the `new()` function.

#### Parameters:

- `real`: The real part of the Complex Number, represented by a `f64` value.
- `imag`: The imaginary part of the Complex Number, represented by a `f64` value.

#### Returns:

A new Complex Number, with the complex type.

#### Equation:

\\[ a + bi \\]

#### Example:

```,norun,rust
use numerilib::Complex;

fn main() {
    let z = Complex::new(1.0, 2.0);

    println!("Complex Number z: {}", z);
}
```

Executes as:

```,norun,rust
>>> Complex Number z: 1+2i
```

### Returning Parts of a Complex Number:

Inorder to return the real or imaginary part of a Complex Number, use the `.real_part()` or `imag_part()` functions.

#### Parameters:

- `&self`: The Complex Number to return the real or imaginary part of.

#### Returns:

The real or imaginary part of the Complex Number, represented by a `f64` value.

#### Returning Real Value Example:

```,norun,rust
use numerilib::Complex;

fn main() {
    let z = Complex::new(1.0, 2.0);

    let real = z.real_part();
    
    println!("Real Part of z: {}", real);
}
```

Executes as:

```,norun,rust
>>> Real Part of z: 1
```

#### Returning Imaginary Value Example:

```,norun,rust
use numerilib::Complex;

fn main() {
    let z = Complex::new(1.0, 2.0);

    let imag = z.imag_part();
    
    println!("Imaginary Part of z: {}", imag);
}
```

Executes as:

```,norun,rust
>>> Imaginary Part of z: 2
```

### The Magnitude of a Complex Number:

The Magnitude of a Complex Number is the distance from the origin to the Complex Number. It is represented by the absolute value of the Complex Number.

#### Parameters:

- `&self`: The Complex Number to return the magnitude of.

#### Returns:

The magnitude of the Complex Number, represented by a `f64` value.

#### Equation:

\\[ \left| z \right| = \sqrt{a^2 + b^2} \\]

#### Example:

```,norun,rust
use numerilib::Complex;

fn main() {
    let z = Complex::new(1.0, 2.0);

    let magnitude = z.magnitude();
    
    println!("Magnitude of z: {}", magnitude);
}
```

Executes as:

```,norun,rust
>>> Magnitude of z: 2.23606797749979 // √5
```

### Complex Arithmetic:

Complex Arithmetic is supported by the `Complex` struct in the `numerilib::Complex` module. *One Quick Notice: Complex Arithmetic can only be done by using operations in the `Complex` type.*

#### Addition Example:

```,norun,rust
use numerilib::Complex;

fn main() {
    let a = Complex::new(3.0, 2.0);
    let b = Complex::new(1.0, 4.0);

    let sum = a + b;
    
    println!("Sum of 3+2i + 1+4i = {}", sum);
}
```

Executes as:

```,norun,rust
>>> Sum of 3+2i + 1+4i = 4+6i
```

#### Subtraction Example:

```,norun,rust
use numerilib::Complex;

fn main() {
    let a = Complex::new(3.0, 2.0);
    let b = Complex::new(1.0, 4.0);

    let difference = a - b;
    
    println!("Difference of 3+2i - 1+4i = {}", difference);
}
```

Executes as:

```,norun,rust
>>> Difference of 3+2i - 1+4i = 2-2i
```

#### Multiplication Example:

```,norun,rust
use numerilib::Complex;

fn main() {
    let a = Complex::new(3.0, 2.0);
    let b = Complex::new(1.0, 4.0);

    let product = a * b;
    
    println!("Product of 3+2i * 1+4i = {}", product);
}
```

Executes as:

```,norun,rust
>>> Product of 3+2i * 1+4i = -5+14i
```

#### Division Example:

```,norun,rust
use numerilib::Complex;

fn main() {
    let a = Complex::new(3.0, 2.0);
    let b = Complex::new(1.0, 4.0);

    let quotient = a / b;
    
    println!("Quotient of 3+2i / 1+4i = {}", quotient);
}
```

Executes as:

```,norun,rust
>>> Quotient of 3+2i / 1+4i = 0.5-0.5i
```

#### Power Raised to a `i32`:

```,norun,rust
use numerilib::Complex;

fn main() {
    let a = Complex::new(3.0, 2.0);

    let power = a.powi(2);
    
    println!("Power of 3+2i ^ 2 = {}", power);
}
```

Executes as:

```,norun,rust
>>> Power of 3+2i ^ 2 = 5+11.999999999999998i
```

#### Power Raised to a `f64`:

```,norun,rust
use numerilib::Complex;

fn main() {
    let a = Complex::new(3.0, 2.0);

    let power = a.powf(2_f64);
    
    println!("Power of 3+2i ^ 2 = {}", power);
}
```

Executes as:

```,norun,rust
>>> Power of 3+2i ^ 2 = 5+11.999999999999998i
```