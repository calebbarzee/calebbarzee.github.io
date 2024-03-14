
// Function to calculate PI
function calculatePI( PI ,  n , sign, num_terms)
{
    // Add for 1000000 terms
    for (let i = 0; i <= num_terms; i++) {
        PI = PI + (sign * (4 / ((n) * (n + 1)
                                * (n + 2))));
 
        // Addition and subtraction
        // of alternate sequences
        sign = sign * (-1);
 
        // Increment by 2 according to formula
        n += 2;
    }
 
    // Return the value of Pi
    document.write("The approximation of Pi is " + PI);
}

// Initialise sum=3, n=2, and sign=1
let PI = 3, n = 2, sign = 1;
 
// Function call
calculatePI(PI, n, sign, 1000);