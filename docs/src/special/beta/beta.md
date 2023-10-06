# Beta Functions

- The beta function is closely related to the gamma function (\\( \Gamma\left( z \right) \\)) and has various applications in mathematics, statistics, and other fields. 

***

### Log Beta:

Uses the definition of Logarithms and the Beta Function to calculate the Logarithm of the Beta Function.

#### Parameters:

- `z1`: The first parameter of the Beta Function.
- `z2`: The second parameter of the Beta Function.

#### Returns:

The Logarithm of the Beta Function of the given numbers.

#### Equation:

\\[ \text{ln}B\left( z_{1},z_{2} \right)=\text{ln}\Gamma\left( z_{1} \right)+\text{ln}\Gamma\left( z_{2} \right)-\text{ln}\Gamma\left( z_{1}+z_{2} \right) \\]

#### Example:

```,norun,rust                                     
use ferrate::special::Beta;             

fn main() {                            
    let z1 = 1_f64;                    
    let z2 = 2_f64;                    
                                       
    let lnbeta = Beta::lnbeta(z1, z2); 
                                       
    println!("The Log Beta Function of {} and {} is: {}", z1, z2, lnbeta);
}
```

Executes as:

```,norun,rust
>>> The Log Beta Function of 1 and 2 is: -0.6931471805616405
```                                     

### Beta Function:
Calculates the Beta Function. The Beta Function is a special function that is closely related to the Gamma Function (\\( \Gamma\left( z \right) \\)). The Beta Function is used in various fields of mathematics and statistics. For example, the Beta Function is used in the definition of the Student's t-distribution and the F-distribution.

#### Parameters:

- `z1`: The first parameter of the Beta Function.
- `z2`: The second parameter of the Beta Function.

#### Returns:

The Beta Function of the given numbers.

#### Equation:

\\[ B\left( z_{1},z_{2} \right)=\frac{\Gamma\left( z_{1} \right)\Gamma\left( z_{2} \right)}{\Gamma\left( z_{1}+z_{2} \right)} \\]

#### Example:

```,norun,rust                                     
use ferrate::special::Beta;             

fn main() {                            
    let z1 = 1_f64;                    
    let z2 = 2_f64;                    
                                       
    let beta = Beta::beta(z1, z2);     
                                       
    println!("The Beta Function of {} and {} is: {}", z1, z2, beta);
}
```

Executes as:

```,norun,rust
>>> The Beta Function of 1 and 2 is: 0.5
```                                     

### Incomplete Beta Function:

Uses the Definition of an Incomplete Beta Function to calculate. The Incomplete Beta Function is a special function that is closely related to the Beta Function (\\( B\left( z_{1},z_{2} \right) \\)). The Incomplete Beta Function is used in various fields of mathematics and statistics. For example, the Incomplete Beta Function is used in the definition of the Student's t-distribution and the F-distribution.

#### Parameters:

- `x`: The integral bound at which the Incomplete Beta Function will be calculated.
- `z1`: The first parameter of the Incomplete Beta Function.
- `z2`: The second parameter of the Incomplete Beta Function.

#### Returns:

The Incomplete Beta Function of the given numbers.

#### Equation:

\\[ I_{x}\left( z_{1},z_{2} \right)=\frac{B_{x}\left( z_{1},z_{2} \right)}{B\left( z_{1},z_{2} \right)} \\]

\\[ B_{x}\left( z_{1},z_{2} \right)={B\left( z_{1},z_{2} \right)}\cdot I_{x}\left( z_{1},z_{2} \right) \\]

#### Example:

```,norun,rust                                            
use ferrate::special::Beta;                

fn main() {                               
    let x = 1_f64 / 7_f64;                 
    let z1 = 0.5_f64;                      
    let z2 = 3_f64;                        
                                           
    let incbeta = Beta::incbeta(x, z1, z2);
                                           
    println!("The Incomplete Beta Function of {}, {} and {} is: {}", x, z1, z2, incbeta);
}
```

Executes as:

```,norun,rust
>>> The Incomplete Beta Function of 0.14285714285714285, 0.5 and 3 is: 0.6870211373344728
```

### The Regularized Incomplete Beta Function:

Uses a Series Expansion of the Incomplete Beta Function.

#### Parameters:

- `x`: The integral bound at which the Regularized Incomplete Beta Function will be calculated.
- `z1`: The first parameter of the Regularized Incomplete Beta Function.
- `z2`: The second parameter of the Regularized Incomplete Beta Function.

#### Returns:

The Regularized Incomplete Beta Function of the given numbers.

#### Equation:

\\[ I_{x}\left( z_{1},z_{2} \right)=\frac{B_{x}\left( z_{1},z_{2} \right)}{B\left( z_{1},z_{2} \right)} \\]
\\[ I_{x}\left( z_{1},z_{2} \right)=\frac{x^{z_{1}}\left( 1-x \right)^{z_{2}}}{z_{1}B\left( z_{1},z_{2} \right)}\left[ 1+\sum_{n=0}^{\infty }\frac{B\left( z_{1}+1,n+1 \right)}{B\left( z_{1}+z_{2},n+1 \right)}x^{n+1} \right] \\]

#### Example:

```,norun,rust                                          
use ferrate::special::Beta;                  

fn main() {                                 
    let x = 1_f64 / 7_f64;                   
    let z1 = 1_f64;                          
    let z2 = 2_f64;                          
                                             
    let regincbeta = Beta::regincbeta(z1, z2, x);
                                             
    println!("The Regularized Incomplete Beta Function of {}, {} and {} is: {}", x, z1, z2, regincbeta);
}
```

Executes as:

```,norun,rust
>>> The Regularized Incomplete Beta Function of 0.14285714285714285, 1 and 2 is: 0.6440823162530317
```

### The Inverse of the Regularized Incomplete Beta Function:

- Uses Newton's Method to Calculate the Inverse of the Regularized Incomplete Beta Function.

#### Parameters:

- `x`: The integral bound at which the Regularized Incomplete Beta Function will be calculated.
- `z1`: The first parameter of the Regularized Incomplete Beta Function.
- `z2`: The second parameter of the Regularized Incomplete Beta Function.

#### Returns:

The Inverse of the Regularized Incomplete Beta Function of the given numbers.

#### Equation:

\\[ I_{x}^{-1}\left( I_{x}\left( z_{1},z_{2} \right) \right)=1 \\]

#### Example:

```,norun,rust                                          
use ferrate::special::Beta;                  

fn main() {                                 
    let z1 = 1_f64;                          
    let z2 = 2_f64;                          
    let x = 0.590401_f64;                    
                                             
    let inverse = Beta::invregincbeta(z1, z2, x);
                                             
    println!("The Inverse of the Regularized Incomplete Beta Function of {}, {} and {} is: {}", x, z1, z2, inverse);
}
```

Executes as:

```,norun,rust
>>> The Inverse of the Regularized Incomplete Beta Function of 0.590401, 1 and 2 is: 0.3600007812492397
```