//Lo que vi en el curso en general, esta comentado por clases, esta ultima parte es de la ultima clase
//Hare una calculadora, dejare el link del repo aqui cuando este lista: 

fn main(){
    //Dos numeros que vamos a sumar
    let numero_1 = 123;
    let numero_2 = 321;

    let suma = numero_1 + numero_2;

    loop {

        //mostrar numeros en pantalla
        println!("La suma es de {} y {} es: ",numero_1,numero_2);

        //pedir al usuario la suma

        let mut suma_usuario = String::new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();

        let suma_usuario_int: i32 = suma_usuario.trim().parse().unwrap();

        if suma_usuario_int == suma {
            println!("El resultado {} es correcto",suma);
            break;
        } else {
            println!("El resultado {} es incorrecto, intentalo denuevo",suma_usuario_int);
        }
    }
}


// fn main(){
//     println!("Years? ");     
//     let mut edad = String::new();     
//     std::io::stdin().read_line(&mut edad).unwrap();
//     //Convertir edad de string a int
//     let edad_int : u8 = edad.trim().parse().unwrap();

//     if edad_int >= 18 && edad_int != 30{
//         println!("Tienes {} años, eres mayor de edad", edad_int);

//         if edad_int > 60{
//             println!("Eres de la tercera edad");
//         }
//     }
//     else if edad_int == 30{
//         println!("Felicidades, tienes 30");
//     }
//     else{
//         println!("Tienes {} años, eres menor de edad", edad_int);
//     }
// }


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

