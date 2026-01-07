// Basic ArcScript Example
// Variables, arithmetic, and functions

var x = 10;
var y = 20;
var sum = x + y;

func multiply(a, b): {
    return a * b;
} end

var product = multiply(x, y);

// Conditional logic
if product > 100 then {
    var message = "Large product!";
} else {
    var message = "Small product";
} end
