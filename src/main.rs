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
    fn residuo(a:f64, b:f64)->f64{
        a%b
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
        let opcion:u64= match entrada.trim().parse() {
            Ok(numero)=>numero,
            Err(_)=>{
                println!("Por favor, ingrese un número para seleccionar una operación...");
                println!();
                continue;
            }
        };
        println!();
        match opcion{
            1=>{
                print!("Por favor, ingrese un número: ");
                let mut entrada1:String=String::new();
                io::stdout().flush().expect("Error en el forzamiento del buffer.");
                io::stdin().read_line(&mut entrada1).expect("Error en la lectura de la línea.");
                let numeroa:f64= match entrada1.trim().parse() {
                    Ok(numero1)=>numero1,
                    Err(_) =>{
                        println!("Por favor, ingrese un número para poder realizar la operación...");
                        println!();
                        continue;
                    }
                };
                println!();

                print!("Por favor, ingrese otro número: ");
                let mut entrada2:String=String::new();
                io::stdout().flush().expect("Error en el forzamiento del buffer.");
                io::stdin().read_line(&mut entrada2).expect("Error en la lectura de la línea.");
                let numerob:f64= match entrada2.trim().parse() {
                    Ok(numero2)=>numero2,
                    Err(_) =>{
                        println!("Por favor, ingrese un número para poder realizar la operación...");
                        println!();
                        continue;
                    }
                }

                let resultado= sumar(numeroa, numerob);
                println!("La suma de {} y de {} es {} unidades.", numeroa, numerob, resultado);
                println!();
            },
            2=>{
                println!("Por favor, ingrese un número: ");
                let mut entrada1:String=String::new();
                io::stdout().flush().expect("Error en el forzamiento del buffer.");
                io::stdin().read_line(&mut entrada1).expect("Error en la lectura de la línea.");
                let numeroa:f64= match entrada1.trim().parse() {
                    Ok(numero1)=>numero1,
                    Err(_) =>{
                        println!("Por favor, ingrese un número para poder realizar la operación...");
                        println!();
                        continue;
                    }
                };
                println!();

                print!("Por favor, ingrese otro número: ");
                let mut entrada2:String=String::new();
                io::stdout().flush().expect("Error en el forzamiento del buffer.");
                io::stdin().read_line(&mut entrada2).expect("Error en la lectura de la línea.");
                let numerob:f64= match entrada2.trim().parse() {
                    Ok(numero2)=>numero2,
                    Err(_) =>{
                        println!("Por favor, ingrese un número para poder realizar la operación...");
                        println!();
                        continue;
                    }
                };

                let resultado= restar(numeroa, numerob);
                println!("La resta de {} y de {} es {} unidades.", numeroa, numerob, resultado);
                println!();
            },
            3=>{
                print!("Por favor, ingrese un número: ");
                let mut entrada1:String=String::new();
                io::stdout().flush().expect("Error en el forzamiento del buffer.");
                io::stdin().read_line(&mut entrada1).expect("Error en la lectura de la línea.");
                let numeroa:f64=entrada1.trim().parse() {
                    Ok(numero1)=>numero1
                    Err(_) =>{
                        println!("Por favor, ingrese un número para poder realizar la operación...");
                        println!();
                        continue;
                    }
                };
                println!();

                print!("Por favor, ingrese otro número: ");
                let mut entrada2:String=String::new();
                io::stdout().flush().expect("Error en el forzamiento del buffer.");
                io::stdin().read_line(&mut entrada2).expect("Error en la lectura de la línea.");
                let numerob:f64=entrada2.trim().parse() {
                    Ok(numero2)=>numero2
                    Err(_) =>{
                        println!("Por favor, ingrese un número para poder realizar la operación...");
                        println!();
                        continue;
                    }
                };

                let resultado= multiplicar(numeroa, numerob);
                println!("La multiplicación de {} y de {} es {} unidades.", numeroa, numerob, resultado);
                println!();
            },
            4=>{
                print!("Por favor, ingrese un número: ");
                let mut entrada1:String=String::new();
                io::stdout().flush().expect("Error en el forzamiento del buffer.");
                io::stdin().read_line(&mut entrada1).expect("Error en la lectura de la línea.");
                let numeroa:f64=match entrada1.trim().parse() {
                    Ok(numero1)=>numero1,
                    Err(_) =>{
                        println!("Por favor, ingrese un número para poder realizar la operación...");
                        println!();
                        continue;
                    }
                };
                println!();

                print!("Por favor, ingrese otro número: ");
                let mut entrada2:String=String::new();
                io::stdout().flush().expect("Error en el forzamiento del buffer.");
                io::stdin().read_line(&mut entrada2).expect("Error en la lectura de la línea.");
                let numerob:f64=match entrada2.trim().parse() {
                    Ok(numero2)=>numero2,
                    Err(_) =>{
                        println!("Por favor, ingrese un número para poder realizar la operación...");
                        println!();
                        continue;
                    }
                };

                let resultado= dividir(numeroa, numerob);
                let resultado2:f64=residuo(numeroa, numerob);
                let mensaje = if numerob==0.00 { String::from("No se puede dividir entre cero, por lo tanto, el resultado está indefinido.")} else {format!("La división de {} y de {} es de {} unidades, y su residuo es de {} unidades. ", numeroa, numerob, resultado, resultado2)};
                println!("{}", mensaje);
                println!();

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
