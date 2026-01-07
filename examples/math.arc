// Math Functions Demo

println("=== Math Functions Demo ===");
println();

// Absolute value
println("1. Absolute values:");
println("  abs(-42):", abs(-42));
println("  abs(3.14):", abs(3.14));
println("  abs(-7.5):", abs(-7.5));
println();

// Min and Max
println("2. Min and Max:");
println("  min(10, 20):", min(10, 20));
println("  max(10, 20):", max(10, 20));
println("  min(-5, -10):", min(-5, -10));
println("  max(3.14, 2.71):", max(3.14, 2.71));
println();

// Rounding functions
println("3. Rounding:");
println("  floor(3.7):", floor(3.7));
println("  ceil(3.2):", ceil(3.2));
println("  round(3.5):", round(3.5));
println("  round(3.4):", round(3.4));
println();

// Power and square root
println("4. Powers and roots:");
println("  pow(2, 8):", pow(2, 8));
println("  pow(3, 3):", pow(3, 3));
println("  sqrt(16):", sqrt(16));
println("  sqrt(2):", sqrt(2));
println();

// Practical examples
println("5. Distance formula:");
var x1 = 0;
var y1 = 0;
var x2 = 3;
var y2 = 4;
var dx = x2 - x1;
var dy = y2 - y1;
var distance = sqrt(pow(dx, 2) + pow(dy, 2));
println("  Distance from (0,0) to (3,4):", distance);
println();

println("6. Average of values:");
var values = 10;
for i = 1, values, 1 do {
    // Example calculation
} end
var avg = 55 / values;  // sum of 1..10 = 55
println("  Average:", avg);
