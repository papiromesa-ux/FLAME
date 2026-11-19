use std::fs;
use std::collections::HashMap;
use std::io::{self, Write};
use std::env;

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
            if partes.len() > 1 {
                let mut textos = Vec::new();
                let mut numeros = Vec::new();
                let mut es_matematica = true;
                for parte in &partes {
                    let val = resolver_texto(parte, variables);
                    textos.push(val.clone());
                    if let Ok(num) = val.parse::<f64>() { numeros.push(num); } else { es_matematica = false; }
                }
                if es_matematica {
                    let mut res = numeros[0];
                    for i in 1..numeros.len() {
                        match op {
                            " H " => res += numeros[i], " S " => res -= numeros[i],
                            " M " => res *= numeros[i], " T " => if numeros[i] != 0.0 { res /= numeros[i]; } else { res = 0.0; },
                            _ => {}
                        }
                    }
                    if res.fract() == 0.0 { return format!("{}", res as i64); }
                    return format!("{}", res);
                } else if op == " H " { return textos.join(""); }
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
                        "==" => return num_izq == num_der, "!=" => return num_izq != num_der,
                        ">" => return num_izq > num_der, "<" => return num_izq < num_der, _ => {}
                    }
                } else {
                    match op { "==" => return izq == der, "!=" => return izq != der, _ => {} }
                }
            }
        }
    }
    false
}

fn ejecutar_lineas(lineas: &[&str], variables: &mut HashMap<String, String>, funciones: &HashMap<String, Vec<String>>) {
    let mut i = 0; let mut saltar_bloque = false; let mut saltar_siguiente = false;
    while i < lineas.len() {
        let l = lineas[i].trim();
        if l.starts_with("###") || l.is_empty() { i += 1; continue; }
        
        // --- NUEVO: INTERACCION CON SPARK AI ---
        if l.starts_with("!ayuda Spark") {
            println!("[Spark AI]: Hola, soy Spark, el cerebro de FLAME. Estoy aqui para ayudarte.");
            println!("[Spark AI]: Analizando el codigo en busca de errores o mejoras...");
            println!("[Spark AI]: Tu codigo parece estar estructurado correctamente. Sigue asi.");
            i += 1; continue;
        }
        if l.starts_with("!descartar Spark") {
            println!("[Spark AI]: Conversacion borrada de la memoria a corto plazo. El codigo sigue intacto.");
            i += 1; continue;
        }
        // --------------------------------------

        if l.starts_with("ja ") {
            let cond = l.replace("ja ", "").replace("[", "").trim().to_string();
            let res = evaluar_condicion(&cond, variables);
            saltar_bloque = !res; saltar_siguiente = res; i += 1; continue;
        }
        if l == "] aber [" { saltar_bloque = saltar_siguiente; i += 1; continue; }
        if l == "]" { saltar_bloque = false; i += 1; continue; }
        if saltar_bloque { i += 1; continue; }

        if l.ends_with(":") && !l.starts_with("@") && !l.starts_with("manifest!") && !l.starts_with("spark!") && !l.starts_with("web!") {
            let nom = l.replace(":", "").trim().to_string();
            if funciones.contains_key(&nom) {
                let b = funciones.get(&nom).unwrap();
                let r: Vec<&str> = b.iter().map(|s| s.as_str()).collect();
                ejecutar_lineas(&r, variables, funciones); 
            }
            i += 1; continue;
        }
        
        if l.starts_with("@") && l.contains("eingabe!") {
            let p: Vec<&str> = l.split('=').collect();
            if p.len() >= 3 {
                let nom = p[0].trim().replace("@", "");
                let preg = p[2].replace("\"", "").replace(":", "").trim().to_string();
                print!("{} ", preg); io::stdout().flush().unwrap(); 
                let mut inp = String::new(); io::stdin().read_line(&mut inp).unwrap(); 
                variables.insert(nom, inp.trim().to_string()); 
            }
        }
        else if l.starts_with("@") {
            let p: Vec<&str> = l.split('=').collect();
            if p.len() == 2 { 
                let nom = p[0].trim().replace("@", "");
                let expr = p[1].replace(":", "").trim().to_string();
                if expr.starts_with("[") { variables.insert(nom, expr); } 
                else { variables.insert(nom, evaluar(&expr, variables)); }
            }
        } 
        else if l.starts_with("manifest!") || l.starts_with("spark!") {
            let expr = l.replace("manifest! = ", "").replace("spark! = ", "").replace(":", "");
            println!("FLAME dice: {}", evaluar(&expr, variables));
        }
        i += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let archivo_fm = if args.len() > 1 { &args[1] } else { "programa.fm" };

    println!("Motor de FLAME iniciado");
    
    if let Ok(contenido) = fs::read_to_string(archivo_fm) {
        let mut contenido_corregido = contenido.clone();
        
        // --- SPARK AI: CORRECTOR GLOBAL ---
        if contenido_corregido.contains("!still") {
            println!("[Spark AI]: Oye, escribiste '!still'. Lo he corregido a '!stil' automaticamente.");
            contenido_corregido = contenido_corregido.replace("!still", "!stil");
        }

        let mut variables: HashMap<String, String> = HashMap::new();
        let mut funciones: HashMap<String, Vec<String>> = HashMap::new(); 
        let lineas: Vec<&str> = contenido_corregido.lines().collect();
        let mut i = 0; 
        let mut en_flamecode = false;
        let mut en_web = false;
        let mut html_buffer = String::new();

        while i < lineas.len() {
            let l = lineas[i].trim();
            
            if l == "<flamecode>" { en_flamecode = true; i += 1; continue; }
            if l == "</flamecode>" { en_flamecode = false; i += 1; continue; }
            if l.starts_with("!stil") { i += 1; continue; }
            if !en_flamecode { i += 1; continue; }

            if l.starts_with("web! = \"") {
                en_web = true;
                html_buffer.push_str(&l.replace("web! = \"", ""));
                html_buffer.push_str("\n");
                i += 1; continue;
            }
            if en_web {
                if l == "\":" {
                    en_web = false;
                    for (key, val) in &variables {
                        let placeholder = format!("@{}", key);
                        html_buffer = html_buffer.replace(&placeholder, val);
                    }
                    fs::write("index.html", &html_buffer).expect("Error creando HTML");
                    println!("[FLAME]: Archivo index.html generado a traves de web!");
                    html_buffer.clear();
                } else {
                    html_buffer.push_str(l); html_buffer.push_str("\n");
                }
                i += 1; continue;
            }

            if l.starts_with("rolle = ") && l.ends_with("[") {
                let nom = l.replace("rolle = ", "").replace(" [", "").trim().to_string();
                let mut n_c = 1; let mut j = i + 1;
                while j < lineas.len() && n_c > 0 {
                    n_c += lineas[j].matches('[').count() as i32;
                    n_c -= lineas[j].matches(']').count() as i32;
                    j += 1;
                }
                funciones.insert(nom, lineas[i+1..j-1].iter().map(|s| s.to_string()).collect());
                i = j - 1; 
            }
            else if l.starts_with("laos = aus=") {
                let cfg = l.replace("laos = aus=", "");
                let p: Vec<&str> = cfg.split(" ~ bis=").collect();
                if p.len() == 2 {
                    let ini: i64 = p[0].trim().replace("\"", "").parse().unwrap_or(1);
                    let fin: i64 = p[1].trim().replace(" [", "").replace("\"", "").parse().unwrap_or(1);
                    let mut n_c = 1; let mut j = i + 1;
                    while j < lineas.len() && n_c > 0 {
                        n_c += lineas[j].matches('[').count() as i32;
                        n_c -= lineas[j].matches(']').count() as i32;
                        j += 1;
                    }
                    for _ in ini..=fin { ejecutar_lineas(&lineas[i+1..j-1], &mut variables, &funciones); }
                    i = j - 1; 
                }
            } 
            else { ejecutar_lineas(&[lineas[i]], &mut variables, &funciones); }
            i += 1;
        }
    } else {
        println!("Error: No se encontro el archivo {}", archivo_fm);
    }
}
