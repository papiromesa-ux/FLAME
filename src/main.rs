use std::fs;
use std::collections::HashMap;
use std::io::{self, Write};
use std::env; // NUEVO: Para leer los argumentos de la terminal

fn resolver_texto(token: &str, variables: &HashMap<String, String>) -> String {
    let token_limpio = token.trim();
    if token_limpio.starts_with("@") {
        let nombre_var = token_limpio.replace("@", "");
        variables.get(&nombre_var).cloned().unwrap_or_else(|| "".to_string())
    } else {
        token_limpio.replace("\"", "")
    }
}

fn evaluar(expresion: &str, variables: &HashMap<String, String>) -> String {
    let operadores = [" H ", " S ", " M ", " T "];
    for op in operadores {
        if expresion.contains(op) {
            let partes: Vec<&str> = expresion.split(op).collect();
            if partes.len() == 2 {
                let izq = resolver_texto(partes[0], variables);
                let der = resolver_texto(partes[1], variables);
                if let (Ok(num_izq), Ok(num_der)) = (izq.parse::<f64>(), der.parse::<f64>()) {
                    let resultado = match op {
                        " H " => num_izq + num_der,
                        " S " => num_izq - num_der,
                        " M " => num_izq * num_der,
                        " T " => if num_der != 0.0 { num_izq / num_der } else { 0.0 },
                        _ => 0.0,
                    };
                    if resultado.fract() == 0.0 { return format!("{}", resultado as i64); }
                    return format!("{}", resultado);
                } else {
                    return format!("{}{}", izq, der);
                }
            }
        }
    }
    resolver_texto(expresion, variables)
}

fn evaluar_condicion(cond: &str, variables: &HashMap<String, String>) -> bool {
    let cond_limpia = cond.trim();
    let valor_directo = resolver_texto(cond_limpia, variables);
    if valor_directo == "wahr" { return true; }
    if valor_directo == "falsch" { return false; }

    let operadores = ["==", "!=", ">", "<"];
    for op in operadores {
        if cond_limpia.contains(op) {
            let partes: Vec<&str> = cond_limpia.split(op).collect();
            if partes.len() == 2 {
                let izq = resolver_texto(partes[0], variables);
                let der = resolver_texto(partes[1], variables);
                if let (Ok(num_izq), Ok(num_der)) = (izq.parse::<f64>(), der.parse::<f64>()) {
                    match op {
                        "==" => return num_izq == num_der,
                        "!=" => return num_izq != num_der,
                        ">" => return num_izq > num_der,
                        "<" => return num_izq < num_der,
                        _ => {}
                    }
                } else {
                    match op {
                        "==" => return izq == der,
                        "!=" => return izq != der,
                        _ => {}
                    }
                }
            }
        }
    }
    false
}

fn ejecutar_lineas(lineas: &[&str], variables: &mut HashMap<String, String>, funciones: &HashMap<String, Vec<String>>) {
    let mut i = 0;
    let mut saltar_bloque = false;
    let mut saltar_siguiente = false;

    while i < lineas.len() {
        let linea_limpia = lineas[i].trim();

        if linea_limpia.starts_with("###") || linea_limpia.is_empty() { i += 1; continue; }

        if linea_limpia.starts_with("ja ") {
            let cond_str = linea_limpia.replace("ja ", "").replace("[", "").trim().to_string();
            let resultado = evaluar_condicion(&cond_str, variables);
            saltar_bloque = !resultado; 
            saltar_siguiente = resultado; 
            i += 1; continue;
        }

        if linea_limpia == "] aber [" {
            saltar_bloque = saltar_siguiente;
            i += 1; continue;
        }

        if linea_limpia == "]" {
            saltar_bloque = false;
            i += 1; continue;
        }

        if saltar_bloque { i += 1; continue; }

        if linea_limpia.ends_with(":") && !linea_limpia.starts_with("@") && !linea_limpia.starts_with("manifest!") {
            let nombre_funcion = linea_limpia.replace(":", "").trim().to_string();
            if funciones.contains_key(&nombre_funcion) {
                let bloque_funcion = funciones.get(&nombre_funcion).unwrap();
                let refs_funcion: Vec<&str> = bloque_funcion.iter().map(|s| s.as_str()).collect();
                ejecutar_lineas(&refs_funcion, variables, funciones); 
            }
            i += 1; continue;
        }

        if linea_limpia.starts_with("@") && linea_limpia.contains("eingabe!") {
            let partes: Vec<&str> = linea_limpia.split('=').collect();
            if partes.len() >= 3 {
                let nombre_var = partes[0].trim().replace("@", "");
                let pregunta = partes[2].replace("\"", "").replace(":", "").trim().to_string();
                print!("{} ", pregunta); 
                io::stdout().flush().unwrap(); 
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap(); 
                let respuesta = input.trim().to_string();
                variables.insert(nombre_var, respuesta); 
            }
        }
        else if linea_limpia.starts_with("@") {
            let partes: Vec<&str> = linea_limpia.split('=').collect();
            if partes.len() == 2 {
                let nombre = partes[0].trim().replace("@", "");
                let expresion = partes[1].replace(":", "");
                variables.insert(nombre, evaluar(&expresion, variables));
            }
        } 
        else if linea_limpia.starts_with("manifest!") {
            let expresion = linea_limpia.replace("manifest! = ", "").replace(":", "");
            println!("FLAME dice: {}", evaluar(&expresion, variables));
        }

        i += 1;
    }
}

fn main() {
    // NUEVO: Leemos los argumentos de la terminal
    let args: Vec<String> = env::args().collect();
    // Si el usuario escribe un nombre de archivo, lo usamos. Si no, usamos programa.fm
    let archivo_fm = if args.len() > 1 { &args[1] } else { "programa.fm" };

    println!("🔥 Motor de FLAME iniciado 🔥");
    println!("Leyendo archivo: {}", archivo_fm);
    
    let mut variables: HashMap<String, String> = HashMap::new();
    let mut funciones: HashMap<String, Vec<String>> = HashMap::new(); 
    
    if let Ok(contenido) = fs::read_to_string(archivo_fm) {
        let lineas: Vec<&str> = contenido.lines().collect();
        let mut i = 0;
        let mut en_flamecode = false;

        while i < lineas.len() {
            let linea_limpia = lineas[i].trim();

            if linea_limpia == "<flamecode>" { en_flamecode = true; i += 1; continue; }
            if linea_limpia == "</flamecode>" { en_flamecode = false; i += 1; continue; }
            if linea_limpia.starts_with("!stil") { i += 1; continue; }
            if !en_flamecode { i += 1; continue; }

            if linea_limpia.starts_with("rolle = ") && linea_limpia.ends_with("[") {
                let nombre_funcion = linea_limpia.replace("rolle = ", "").replace(" [", "").trim().to_string();
                
                let mut nivel_corchetes = 1;
                let mut j = i + 1;
                while j < lineas.len() && nivel_corchetes > 0 {
                    let l = lineas[j].trim();
                    let abiertos = l.matches('[').count() as i32;
                    let cerrados = l.matches(']').count() as i32;
                    nivel_corchetes += abiertos;
                    nivel_corchetes -= cerrados;
                    j += 1;
                }
                let fin_funcion = j - 1;

                let bloque_interno = &lineas[i+1..fin_funcion];
                let bloque_strings: Vec<String> = bloque_interno.iter().map(|s| s.to_string()).collect();
                funciones.insert(nombre_funcion, bloque_strings);

                i = fin_funcion; 
            }
            else if linea_limpia.starts_with("laos = aus=") {
                let config = linea_limpia.replace("laos = aus=", "");
                let partes: Vec<&str> = config.split(" ~ bis=").collect();
                if partes.len() == 2 {
                    let inicio: i64 = partes[0].trim().replace("\"", "").parse().unwrap_or(1);
                    let fin: i64 = partes[1].trim().replace(" [", "").replace("\"", "").parse().unwrap_or(1);
                    let mut nivel_corchetes = 1;
                    let mut j = i + 1;
                    while j < lineas.len() && nivel_corchetes > 0 {
                        let l = lineas[j].trim();
                        let abiertos = l.matches('[').count() as i32;
                        let cerrados = l.matches(']').count() as i32;
                        nivel_corchetes += abiertos;
                        nivel_corchetes -= cerrados;
                        j += 1;
                    }
                    let fin_bucle = j - 1;
                    let bloque_interno = &lineas[i+1..fin_bucle];
                    for _ in inicio..=fin {
                        ejecutar_lineas(bloque_interno, &mut variables, &funciones); 
                    }
                    i = fin_bucle; 
                }
            } 
            else {
                ejecutar_lineas(&[lineas[i]], &mut variables, &funciones); 
            }
            i += 1;
        }
    } else {
        println!("❌ Error: No se encontró el archivo {}", archivo_fm);
    }
}
