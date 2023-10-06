use std::ops::{Add, Div, Mul, Sub};

/// A module containing Complex numbers and their operations.
#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    /// Create a new Complex number given a real and imaginary part.
    ///
    /// # Format:
    /// `a + bi`
    /// - where `a` is the real part
    /// - where `b` is the imaginary part
    ///
    /// # Parameters:
    ///
    /// - `real`: the real part of the Complex number -> `a`
    /// - `imag`: the imaginary part of the Complex number -> `b`
    ///
    /// # Returns:
    ///
    /// A new Complex number with the given real and imaginary parts.
    ///
    /// # Example:
    ///
    /// ```
    /// use numerilib::
    pub fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }

    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    pub fn powi(&self, n: i32) -> Complex {
        let magnitude = self.magnitude().powi(n);
        let angle = n as f64 * (self.imag / self.real).atan();
        let real = magnitude * angle.cos();
        let imag = magnitude * angle.sin();
        Complex::new(real, imag)
    }

    pub fn powf(&self, n: f64) -> Complex {
        let magnitude = self.magnitude().powf(n);
        let angle = n * (self.imag / self.real).atan();
        let real = magnitude * angle.cos();
        let imag = magnitude * angle.sin();
        Complex::new(real, imag)
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}i",
            self.real,
            if self.imag >= 0.0 { '+' } else { '-' },
            self.imag.abs()
        )
    }
}

// Implement addition for Complex numbers
impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

// Implement subtraction for Complex numbers
impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}

// Implement multiplication for Complex numbers
impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        Complex::new(real, imag)
    }
}

// Implement division for Complex numbers
impl Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let denominator = other.real * other.real + other.imag * other.imag;
        let real = (self.real * other.real + self.imag * other.imag) / denominator;
        let imag = (self.imag * other.real - self.real * other.imag) / denominator;
        Complex::new(real, imag)
    }
}
