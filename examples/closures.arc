// Closure Example
// Functions can capture variables from their defining scope

var counter = 0;

func makeCounter(): {
    func increment(): {
        counter = counter + 1;
        return counter;
    } end
    return increment;
} end

var inc = makeCounter();
var val1 = inc();  // counter = 1
var val2 = inc();  // counter = 2

// Closure with local capture
func makeAdder(x): {
    func add(y): {
        return x + y;
    } end
    return add;
} end

var add5 = makeAdder(5);
var result = add5(10);  // result = 15
