use std::fs;
use std::collections::HashMap;

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
    let operadores = ["==", "!=", ">", "<"];
    for op in operadores {
        if cond.contains(op) {
            let partes: Vec<&str> = cond.split(op).collect();
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

fn ejecutar_lineas(lineas: &[&str], variables: &mut HashMap<String, String>) {
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

        if linea_limpia.starts_with("@") {
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
    println!("FLAME MOTOR");
    let mut variables: HashMap<String, String> = HashMap::new();
    
    if let Ok(contenido) = fs::read_to_string("programa.fm") {
        let lineas: Vec<&str> = contenido.lines().collect();
        let mut i = 0;
        let mut en_flamecode = false;

        while i < lineas.len() {
            let linea_limpia = lineas[i].trim();

            if linea_limpia == "<flamecode>" { en_flamecode = true; i += 1; continue; }
            if linea_limpia == "</flamecode>" { en_flamecode = false; i += 1; continue; }
            if linea_limpia.starts_with("!stil") { i += 1; continue; }
            if !en_flamecode { i += 1; continue; }

            if linea_limpia.starts_with("laos = aus=") {
                let config = linea_limpia.replace("laos = aus=", "");
                let partes: Vec<&str> = config.split(" ~ bis=").collect();
                
                if partes.len() == 2 {
                    let inicio: i64 = partes[0].trim().replace("\"", "").parse().unwrap_or(1);
                    let fin: i64 = partes[1].trim().replace(" [", "").replace("\"", "").parse().unwrap_or(1);

                    let mut nivel_corchetes = 1;
                    let mut j = i + 1;
                    
                    // CEREBRO 
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
                        ejecutar_lineas(bloque_interno, &mut variables);
                    }
                    
                    i = fin_bucle; 
                }
            } 
            else {
                ejecutar_lineas(&[lineas[i]], &mut variables);
            }

            i += 1;
        }
    } else {
        println!(" Error: No se encontró el archivo programa.fm");
    }
}
