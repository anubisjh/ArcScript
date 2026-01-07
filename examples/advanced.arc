// Advanced Features Showcase
// This example demonstrates closures, tables, recursion, and more

println("=== Advanced ArcScript Features ===");
println();

// 1. Closures and Higher-Order Functions
println("1. Closures:");
func makeCounter(start): {
    var count = start;
    func increment(): {
        count = count + 1;
        return count;
    } end
    return increment;
} end

var counter1 = makeCounter(0);
var counter2 = makeCounter(100);

println("counter1():", counter1());  // 1
println("counter1():", counter1());  // 2
println("counter2():", counter2());  // 101
println();

// 2. Nested Tables
println("2. Nested Tables:");
var game = {
    title: "Adventure Quest",
    settings: {
        resolution: "1920x1080",
        volume: 80
    },
    player: {
        name: "Hero",
        stats: {
            hp: 100,
            mp: 50,
            level: 10
        }
    }
};

println("Game title:", game.title);
println("Volume:", game["settings"]["volume"]);
println("Player HP:", game.player.stats.hp);
println();

// 3. Functions as Values
println("3. Functions as Values:");
func add(a, b): {
    return a + b;
} end

func multiply(a, b): {
    return a * b;
} end

func apply(operation, x, y): {
    return operation(x, y);
} end

println("apply(add, 5, 3):", apply(add, 5, 3));
println("apply(multiply, 5, 3):", apply(multiply, 5, 3));
println();

// 4. Table Manipulation
println("4. Table Manipulation:");
var inventory = {
    sword: 1,
    potion: 5,
    gold: 100
};

println("Initial inventory size:", len(inventory));
println("Gold:", inventory.gold);

// 5. Recursive Functions
println();
println("5. Recursion:");
func fibonacci(n): {
    if n <= 1 then {
        return n;
    } end
    return fibonacci(n - 1) + fibonacci(n - 2);
} end

println("fibonacci(10):", fibonacci(10));
println();

// 6. String Operations
println("6. String Operations:");
var message = "Hello, World!";
println("Message:", message);
println("Length:", len(message));
println("Type:", type(message));
