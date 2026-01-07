// Control Flow Example

var i = 0;

// While loop
while i < 5 do {
    i = i + 1;
} end

// If-elif-else
var score = 85;

if score >= 90 then {
    var grade = "A";
} elif score >= 80 then {
    var grade = "B";
} elif score >= 70 then {
    var grade = "C";
} else {
    var grade = "F";
} end

// Nested loops
var outer = 0;
while outer < 3 do {
    var inner = 0;
    while inner < 3 do {
        inner = inner + 1;
    } end
    outer = outer + 1;
} end
