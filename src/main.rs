fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    println!("{}", fibonacci(40));
}


//the fn fibonacci is recursive, it calls itself until it reaches the base case, which is when n is 0 or 1.
//The base case returns the value 0 or 1, and the recursive case returns the sum of the previous two numbers.