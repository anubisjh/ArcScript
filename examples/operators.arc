// New Operators Demo

println("=== New Operators Demo ===");
println();

// 1. Modulo operator (%)
println("1. Modulo operator:");
println("  10 % 3 =", 10 % 3);      // 1
println("  15 % 4 =", 15 % 4);      // 3
println("  20 % 5 =", 20 % 5);      // 0
println("  7 % 2 =", 7 % 2);        // 1 (odd check)
println();

// Check if number is even or odd
func isEven(n): {
    return n % 2 == 0;
} end

println("2. Even/Odd check:");
println("  isEven(10):", isEven(10));
println("  isEven(7):", isEven(7));
println();

// 3. Assignment operators
println("3. Assignment operators:");
var x = 10;
println("  x = 10");

x = x + 5;  // Regular assignment
println("  x = x + 5  =>  x =", x);

x = 10;  // Reset
x += 5;  // Compound assignment
println("  x = 10, then x += 5  =>  x =", x);

x = 10;  // Reset
x -= 3;
println("  x = 10, then x -= 3  =>  x =", x);

x = 10;  // Reset
x *= 2;
println("  x = 10, then x *= 2  =>  x =", x);

x = 10;  // Reset
x /= 2;
println("  x = 10, then x /= 2  =>  x =", x);
println();

// 4. Logical NOT with !
println("4. Logical NOT (!):");
var flag = true;
println("  flag = true");
println("  !flag =", !flag);
println("  not flag =", not flag);  // Both work!

var a = 5;
var b = 10;
println("  a = 5, b = 10");
println("  !(a > b) =", !(a > b));
println("  !(a == b) =", !(a == b));
println();

// 5. Practical example: FizzBuzz
println("5. FizzBuzz (1-15):");
for i = 1, 15, 1 do {
    if i % 15 == 0 then {
        println("  FizzBuzz");
    } elif i % 3 == 0 then {
        println("  Fizz");
    } elif i % 5 == 0 then {
        println("  Buzz");
    } else {
        println("  ", i);
    } end
} end
