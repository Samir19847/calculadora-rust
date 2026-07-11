use std::io;
use std::io::Write;

fn main() {

    fn sumar(a:f64, b:f64)->f64{
        a+b
    }
    fn restar(a:f64, b:f64)->f64{
        a-b
    }
    fn multiplicar(a:f64, b:f64)->f64{
        a*b
    }
    fn dividir(a:f64, b:f64)->f64{
        a/b
    }
    
    loop {
        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
        println!("-_-          ¡Bienvenido al sistema!          -_-");
        println!("-_-             Calculadora Rust              -_-");
        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
        println!();
        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
        println!("-_-               MENÚ PRINCIPAL              -_-");
        println!("-_-                1. Sumar                   -_-");
        println!("-_-                2. Restar                  -_-");
        println!("-_-                3. Multiplicar             -_-");  
        println!("-_-                4. Dividir                 -_-");
        println!("-_-                5. Salir                   -_-");
        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
        println!();
        print!("Por favor, selecciona tu operación: ");
        let mut entrada:String=String::new();
        io::stdout().flush().expect("Error en el forzamiento del buffer.");
        io::stdin().read_line(&mut entrada).expect("Error en la lectura de la línea.");
        let opcion:u64=entrada.trim().parse().expect("Error en la conversión de datos.");
        match opcion{
            1=>{

            },
            2=>{

            },
            3=>{

            },
            4=>{

            },
            5=>{
                println!("Saliendo del programa..."); break;
            },
            _=>{
                println!("Opción inválida, por favor, ingrese una opción correcta: ");
                println!();
            }
        }

    }
}
