fn main(){
    println!("Years? ");     
    let mut edad = String::new();     
    std::io::stdin().read_line(&mut edad).unwrap();
    //Convertir edad de string a int
    let edad_int : u8 = edad.trim().parse().unwrap();


    if edad_int >= 18 && edad_int != 30{
        println!("Tienes {} años, eres mayor de edad", edad_int);

        if edad_int > 60{
            println!("Eres de la tercera edad");
        }
    }
    else if edad_int == 30{
        println!("Felicidades, tienes 30");
    }
    else{
        println!("Tienes {} años, eres menor de edad", edad_int);
    }



}




// fn main(){
//     //Formulario to csm
//     println!("Hola cual es tu nombre? ");
//     let mut nombre = String::new();
//     std::io::stdin().read_line(&mut nombre).unwrap();
//     nombre = nombre.trim().to_string();


//     println!("Holaa de que pais veni boluo csm blu daba diba duba duba da diba? ");
//     let mut pais = String::new();
//     std::io::stdin().read_line(&mut pais).unwrap();
//     pais = pais.trim().to_string();

//     println!("Entonces te llamas {} y eres de {} csm",nombre,pais);
    
// }
 



// fn fibonacci(n: u128) -> u128 {
//     if n == 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     } else {
//         return fibonacci(n - 1) + fibonacci(n - 2);
//     }
// }

// fn main() {
//     println!("La suma de los numeros fibonacci 20 veces es {}", fibonacci(20));


// //the fn fibonacci is recursive, it calls itself until it reaches the base case, which is when n is 0 or 1.
// //The base case returns the value 0 or 1, and the recursive case returns the sum of the previous two numbers.

//     println!("Your name? ");
//     let mut nombre = String::new();

//     std::io::stdin().read_line(&mut nombre).unwrap();
//     nombre = nombre.trim().to_string();

//     println!("Years? ");
//     let mut edad = String::new();
//     std::io::stdin().read_line(&mut edad).unwrap();
//     //Convertir edad de string a int
//     let edad_int : u8 = edad.trim().parse().unwrap();
    

//     println!("Welcome {}, you have {} years", nombre, edad_int);



// }

