fn fibonacci(n: u128) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    println!("La suma de los numeros fibonacci 20 veces es {}", fibonacci(20));


//the fn fibonacci is recursive, it calls itself until it reaches the base case, which is when n is 0 or 1.
//The base case returns the value 0 or 1, and the recursive case returns the sum of the previous two numbers.

    println!("Your name? ");
    let mut nombre = String::new();

    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    println!("Years? ");
    let mut edad = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    //Convertir edad de string a int
    let edad_int : u8 = edad.trim().parse().unwrap();
    

    println!("Welcome {}, you have {} years", nombre, edad_int);


}

