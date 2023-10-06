# Error Functions

- Error Function and its relating functions

***

### The Error Function/Erf:
Uses the Maclaurin Series Expansion of the Error Function. The Error Function is the integral of the Gaussian Function. The Error Function is used in statistics to calculate the probability of a given value of a normal distribution.

#### Parameters:

- `z`: The number for which the Error Function will be calculated.

#### Returns:

The Error Function of the given number.

#### Equation:

\\[ \text{erf}\left( z \right)=\frac{2}{\sqrt{\pi}}\int_{0}^{z}e^{-t^{2}}dt \\]
\\[ \text{erf}\left( z \right)=\frac{2}{\sqrt{\pi}}\sum_{n=0}^{\infty }\frac{\left( -1 \right)^{n}z^{2x+1}}{n!\left( 2n+1 \right)} \\]

#### Example:

```,norun,rust                                 
use ferrate::special::Error;        
                         
fn main() {                
    let z = 2_f64;        
                          
    let erf = Error::erf(z);
                          
    println!("The Error Function of {} is: {}", z, erf);
}
```

Executes as:

```,norun,rust
>>> The Error Function of 2 is: 0.9999999845946841
```

### The Complementary Error Function/Erfc:

Uses the definition of the Complementary Error Function to compute. The Complementary Error Function is the integral of the Gaussian Function.

#### Parameters:
    
- `z`: The number for which the Complementary Error Function will be calculated.

#### Returns:

The Complementary Error Function of the given number.

#### Equation:

\\[ \text{erfc}\left( z \right)=1-\text{erf}\left( z \right) \\]

#### Example:

```,norun,rust                                              
use ferrate::special::Error;                     

fn main() {                                        
    let z = 2_f64;                                 
                                                     
    let erfc = Error::erfc(z);                      
                                                     
    println!("The Complementary Error Function of {} is: {}", z, erfc);
}
```

Executes as:

```,norun,rust
>>> The Complementary Error Function of 2 is: 0.000000015405315911820594
```

### The Inverse Error Function
Approximates the Inverse of the Error Function.

#### Parameters:

- `z`: The number for which the Inverse Error Function will be calculated.

#### Returns:

The Inverse Error Function of the given number.

#### Equation:

\\[ z==\text{erf}^{-1}\left( w \right) \\]
\\[ w==\text{erf}\left( z \right) \\]
      
#### Example:

```,norun,rust                                    
use ferrate::special::Error;           

fn main() {                              
    let z = 0.975;                       
                                         
    let inverf = Error::inverf(z);       
                                         
    println!("The Inverse Error Function of {} is: {}", z, inverf);
}
```

Executes as:

```,norun,rust
>>> The Inverse Error Function of 0.975 is: 1.5849110680594818
```