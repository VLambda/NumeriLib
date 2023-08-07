# Error Functions

- Error Function and its relating functions

***

## Ferrate Functions:

### The Error Function/Erf:

- The Maclaurin Series Expansion of the Error Function

\\[ \text{erf}\left( z \right)=\frac{2}{\sqrt{\pi}}\int_{0}^{z}e^{-t^{2}}dt=\frac{2}{\sqrt{\pi}}\sum_{n=0}^{\infty }\frac{\left( -1 \right)^{n}z^{2x+1}}{n!\left( 2n+1 \right)} \\]

#### Example:

```,norun,rust                                 
use ferrate::special::Error;        
                                    
let bound = 4_f64;                  
let erf = Error::erf(bound);        
                                    
assert_eq!(erf, 0.9999999845946841);
```

### The Complementary Error Function/Erfc:

- The Definition of the Complementary Error Function

\\[ \text{erfc}\left( z \right)=1-\text{erf}\left( z \right) \\]

#### Example:

```,norun,rust                                              
use ferrate::special::Error;                     
                                                 
let z = 4_f64;                                   
                                                 
let erfc = Error::erfc(z);                       
                                                 
assert_eq!(erfc, 0.000000015405315911820594_f64);
```

### The Inverse Error Function

- Approximates the Inverse of the Error Function

\\[ z==\text{erf}^{-1}\left( w \right)/;w==\text{erf}\left( z \right) \\]
      
#### Example:

```,norun,rust                                    
use ferrate::special::Error;           
                                       
let x = 0.975;                         
let inverf = Error::inverf(x);         
                                       
assert_eq!(inverf, 1.5849110680594818);
```                                    