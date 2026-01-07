// String Functions Demo

println("=== String Functions Demo ===");
println();

// Length (already had this)
println("1. String length:");
var message = "Hello, World!";
println("  len(\"Hello, World!\"):", len(message));
println();

// Substring
println("2. Substring:");
var text = "ArcScript";
println("  substring(\"ArcScript\", 0, 3):", substring(text, 0, 3));
println("  substring(\"ArcScript\", 3, 9):", substring(text, 3, 9));
println("  substring(\"ArcScript\", 0, 9):", substring(text, 0, 9));
println();

// Contains
println("3. String search:");
println("  contains(\"Hello World\", \"World\"):", contains("Hello World", "World"));
println("  contains(\"Hello World\", \"Goodbye\"):", contains("Hello World", "Goodbye"));
println("  contains(\"test@email.com\", \"@\"):", contains("test@email.com", "@"));
println();

// Case conversion
println("4. Case conversion:");
var name = "Alice";
println("  toUpper(\"Alice\"):", toUpper(name));
println("  toLower(\"HELLO\"):", toLower("HELLO"));
println("  toLower(\"CamelCase\"):", toLower("CamelCase"));
println();

// Practical example
println("5. Email validation (simple):");
var email = "user@example.com";
if contains(email, "@") and contains(email, ".") then {
    println("  ", email, "looks valid!");
} else {
    println("  Invalid email");
} end
println();

// String concatenation (already worked)
println("6. Building strings:");
var firstName = "John";
var lastName = "Doe";
var fullName = firstName + " " + lastName;
println("  Full name:", fullName);
println("  Uppercase:", toUpper(fullName));
println("  Lowercase:", toLower(fullName));
