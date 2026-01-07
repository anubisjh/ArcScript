// ArcScript Built-in Functions Demo

// Print functions
println("=== Built-in Functions Demo ===");
println();

// Type checking
var x = 42;
var y = 3.14;
var name = "Alice";
var flag = true;

println("Type checks:");
println("type(42):", type(x));
println("type(3.14):", type(y));
println("type(\"Alice\"):", type(name));
println("type(true):", type(flag));
println();

// Length function
println("Length checks:");
println("len(\"Hello\"):", len("Hello"));
println("len(\"ArcScript\"):", len("ArcScript"));
println();

// Type conversions
println("Type conversions:");
println("int(3.14):", int(3.14));
println("int(\"42\"):", int("42"));
println("int(true):", int(true));
println("int(false):", int(false));
println("float(42):", float(42));
println("float(\"3.14\"):", float("3.14"));
println("str(42):", str(42));
println("str(3.14):", str(3.14));
println("str(true):", str(true));
println();

// Print without newline
print("This is ");
print("printed ");
println("on one line!");

// Table length
var player = {
    name: "Hero",
    hp: 100,
    level: 5
};

println();
println("Table length:");
println("len(player):", len(player));
