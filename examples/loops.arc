// For Loop Examples

println("=== For Loop Examples ===");
println();

// Basic for loop
println("1. Basic counting:");
for i = 1, 5, 1 do {
    println("  Count:", i);
} end
println();

// Countdown
println("2. Countdown:");
for i = 5, 1, -1 do {
    println("  T-minus:", i);
} end
println();

// Step by 2
println("3. Even numbers:");
for i = 0, 10, 2 do {
    println("  ", i);
} end
println();

// Using for loop result
println("4. Sum with for loop:");
var sum = 0;
for i = 1, 10, 1 do {
    sum = sum + i;
} end
println("  Sum of 1-10:", sum);
println();

// Break in for loop
println("5. Break example:");
for i = 1, 100, 1 do {
    if i > 5 then {
        break;
    } end
    println("  ", i);
} end
println();

// Nested for loops
println("6. Multiplication table:");
for i = 1, 3, 1 do {
    for j = 1, 3, 1 do {
        print(i * j, " ");
    } end
    println();
} end
