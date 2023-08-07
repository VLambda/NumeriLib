# Gamma Functions

- All the Gamma Functions
***

## Ferate Functions:

### Stirling's Approximation:

- The Definition of Stirling's Approximation

\\[ n!\sim \sqrt{2\pi n}\left( \frac{n}{e} \right)^{n} \\]

#### Example:

```,norun,rust                                          
use ferrate::special::Gamma;                 
                                             
let n = 2_f64;                               
let stirling = Gamma::stirling(n);           
                                             
assert_eq!(stirling, 1.9190043514889832_f64);
```

### Log Gamma Lanczos:

- the Log Gamma Function using Lanczos Approximation

\\[ \text{ln}\Gamma\left( x \right) \\]
    
#### Example:

```,norun,rust                                       
use ferrate::special::Gamma;              
                                          
let n = 6_f64;                            
let lanczos_ln = Gamma::lanczosln(n);     
                                          
assert_eq!(lanczos_ln, 4.787491742764145);
```

### Gamma Function Lanczos:

- The Gamma Function using Lanczos's Approximation

\\[ \Gamma\left( x \right)=\left( x-1 \right)!=\int_{0}^{\infty }t^{x-1}e^{-t}dt=e^{ln\Gamma\left( x \right)} \\]

#### Example:

```,norun,rust                             
use ferrate::special::Gamma;    
                                
let n = 6_f64;                  
let lanczos = Gamma::lanczos(n);
                                
assert_eq!(lanczos, 120_f64);   
```

### Incomplete Gamma Function (Lower)

- Calculates the Lower Incomplete Gamma Function using its Series Representation

\\[ \gamma\left( s,x \right)=\int_{x}^{\infty }t^{s-1}e^{-t}dt=x^{s}\sum_{k=0}^{\infty }\frac{\left( -1 \right)^{k}x^{k}}{\left( s+k \right)k!} \\]

#### Example:

```,norun,rust                                   
use ferrate::special::Gamma;               
                                           
let bound = 3_f64;                         
let x = 1_f64;                             
                                           
let gamma = Gamma::incgamma(bound, x);     
                                           
assert_eq!(gamma, 0.16060279414278839_f64);                                      
```                                        

### Incomplete Gamma Function (Upper)

- Uses the Defintion of the Upper Incomplete Gamma Function

\\[ \Gamma\left( s,x \right)+\gamma\left( s,x \right)=\Gamma\left( x \right)/; \Gamma\left( s,x \right)=\Gamma\left( x \right)-\gamma\left( s,x \right) \\]

#### Example:

```,norun,rust                                       
use ferrate::special::Gamma;              
                                          
let a = 3_f64;                            
let x = 1_f64;                            
                                          
let gamma = Gamma::incgammac(a, x);       
                                          
assert_eq!(gamma, 1.8393972058572117_f64);
```    

### The Regularized Incomplete Gamma Function

- Uses the Definiton of the Regularized Incomplete Gamma Function

\\[ \text{P}\left( s,x \right)=\frac{\gamma\left( s,x \right)}{\Gamma\left( x \right)} \\]

#### Example:

```,norun,rust                                            
use ferrate::special::Gamma;                   
                                               
let bound = 5_f64;                             
let x = 2_f64;                                 
                                               
let reggamma = Gamma::reggamma(bound, x);      
                                               
assert_eq!(reggamma, 0.052653017343711174_f64);
```

### CDF for Poisson Random Variables

- Uses the Definition of the Poisson Random Variables

\\[ \text{Q}\left( s,x \right)=\frac{\Gamma\left( s,x \right)}{\Gamma\left( x \right)}=1-\text{P}\left( s,x \right) \\]

#### Example:

```,norun,rust                                           
use ferrate::special::Gamma;                  
                                              
let bound = 5_f64;                            
let x = 2_f64;                                
                                              
let reggammac = Gamma::reggammac(bound, x);   
                                              
assert_eq!(reggammac, 0.9473469826562888_f64);
```                                           