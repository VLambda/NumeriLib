use std::ops::{Add, Div, Mul, Sub};

/// A module containing Complex numbers and their operations.
#[derive(Debug, Clone, Copy)]
pub struct Complex {
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
    /// use numerilib::Complex;
    ///
    /// let z = Complex::new(1.0, 2.0);
    ///
    /// println!("Complex Number z: {}", z);
    /// ```
    /// * * *
    pub fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }

    /// Returns the real part of a Complex number.
    ///
    /// # Parameters:
    ///
    /// - `&self`: the Complex number to get the real part of
    ///
    /// # Returns:
    ///
    /// The real part of the Complex number.
    ///
    /// # Example:
    ///
    /// ```
    /// use numerilib::Complex;
    ///
    /// let z = Complex::new(1.0, 2.0);
    ///
    /// let real_part = z.real_part();
    ///
    /// println!("Real part of {}: {}", z, real_part);
    ///
    /// ```
    /// * * *
    pub fn real_part(&self) -> f64 {
        self.real
    }

    /// Returns the imaginary part of a Complex number.
    ///
    /// # Parameters:
    ///
    /// - `&self`: the Complex number to get the imaginary part of
    ///
    /// # Returns:
    ///
    /// The imaginary part of the Complex number.
    ///
    /// # Example:
    ///
    /// ```
    /// use numerilib::Complex;
    ///
    /// let z = Complex::new(1.0, 2.0);
    ///
    /// let imag_part = z.imag_part();
    ///
    /// println!("Imaginary part of {}: {}", z, imag_part);
    /// ```
    /// * * *
    pub fn imag_part(&self) -> f64 {
        self.imag
    }

    /// Calculates the magnitude of a Complex number.
    ///
    /// # Parameters:
    ///
    /// - `&self`: the Complex number to calculate the magnitude of
    ///
    /// # Returns:
    ///
    /// The magnitude of the Complex number.
    ///
    /// # Example:
    ///
    /// ```
    /// use numerilib::Complex;
    ///
    /// let z = Complex::new(1.0, 2.0);
    ///
    /// let magnitude = z.magnitude();
    ///
    /// println!("Magnitude of {}: {}", z, magnitude);
    /// ```
    /// * * *
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    /// Calculates the Complex number raised to the power of an integer.
    ///
    /// # Parameters:
    ///
    /// - `&self`: the Complex number to be raised to the power of an integer
    /// - `n`: the integer to raise the Complex number to
    ///
    /// # Returns:
    ///
    /// The Complex number raised to the power of an integer.
    ///
    /// # Example:
    ///
    /// ```
    /// use numerilib::Complex;
    ///
    /// let z = Complex::new(1.0, 2.0);
    ///
    /// let z_to_the_power_of_3 = z.powi(3);
    ///
    /// println!("{}^3 = {}", z, z_to_the_power_of_3);
    /// ```
    /// * * *
    pub fn powi(&self, n: i32) -> Complex {
        let magnitude = self.magnitude().powi(n);
        let angle = n as f64 * (self.imag / self.real).atan();
        let real = magnitude * angle.cos();
        let imag = magnitude * angle.sin();
        Complex::new(real, imag)
    }

    /// Calculates the Complex number raised to the power of a float.
    ///
    /// # Parameters:
    ///
    /// - `&self`: the Complex number to be raised to the power of a float
    /// - `n`: the float to raise the Complex number to
    ///
    /// # Returns:
    ///
    /// The Complex number raised to the power of a float.
    ///
    /// # Example:
    ///
    /// ```
    /// use numerilib::Complex;
    ///
    /// let z = Complex::new(1.0, 2.0);
    ///
    /// let z_to_the_power_of_3 = z.powf(3.0);
    ///
    /// println!("{}^3 = {}", z, z_to_the_power_of_3);
    /// ```
    /// * * *
    pub fn powf(&self, n: f64) -> Complex {
        let magnitude = self.magnitude().powf(n);
        let angle = n * (self.imag / self.real).atan();
        let real = magnitude * angle.cos();
        let imag = magnitude * angle.sin();
        Complex::new(real, imag)
    }

    /// Complex Arithmetic:
    ///
    /// Regular Arithmetic but with Complex Numbers.
    ///
    /// # Example:
    ///
    /// ```
    /// use numerilib::Complex;
    ///
    /// let z1 = Complex::new(1.0, 2.0);
    ///
    /// let z2 = Complex::new(3.0, 4.0);
    ///
    /// let complex_sum = z1 + z2;
    /// let complex_minus = z1 - z2;
    /// let complex_product = z1 * z2;
    /// let complex_division = z1 / z2;
    ///
    /// println!("{} + {} = {}", z1, z2, complex_sum);
    /// println!("{} - {} = {}", z1, z2, complex_minus);
    /// println!("{} * {} = {}", z1, z2, complex_product);
    /// println!("{} / {} = {}", z1, z2, complex_division);
    /// ```
    /// * * *
    pub fn arithmetic() {
        /*
            Why is this here?

            Well this is here because I needed to find a way to document the arithmetic functions
            of the Complex Numbers.

            This function has no logic/code in it, if you want to see the code for the arithmetic,
            scroll down and you will see the implimentation blocks of the arithmetic functions.
        */
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
