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

fn procesar_nml(contenido: &str) -> (String, String) {
    let mut html = String::new(); let mut css = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n<link rel=\"stylesheet\" href=\"style.css\">\n");
    let mut en_stimob = false;
    for linea in contenido.lines() {
        let l = linea.trim();
        if l.is_empty() || l.starts_with("!stil") || l == "<01>" { continue; }
        if l == "</01>" { html.push_str("</body>\n</html>"); continue; }
        if l == "<02>" { continue; }
        if l == "</02>" { html.push_str("</head>\n<body style='background-color: #222; color: white; font-family: Arial;'>\n"); continue; }
        if l == "<03>" || l == "</03>" { continue; }
        if l == "<StiMoB>" { en_stimob = true; continue; }
        if l == "</StiMoB>" { en_stimob = false; continue; }
        if en_stimob {
            if l.starts_with("[klasse=") {
                let start_q = l.find("\"").unwrap_or(0);
                let end_q = l[start_q+1..].find("\"").unwrap_or(0) + start_q + 1;
                let class_name = &l[start_q+1..end_q];
                let props_str = l[end_q+1..].replace(":", "").replace("]", "").trim().to_string();
                css.push_str(&format!(".{} {{\n", class_name));
                for prop in props_str.split(',') {
                    let p = prop.trim();
                    if p.starts_with("farbe=") {
                        let color = p.replace("farbe=", "");
                        let css_color = match color.as_str() {
                            "feuer" => "#ff4500", "schwarz" => "#000000", "weiss" => "#ffffff", "blau" => "#0000ff", _ => &color,
                        };
                        css.push_str(&format!("  color: {};\n", css_color));
                    } else if p.starts_with("size=") {
                        css.push_str(&format!("  font-size: {}px;\n", p.replace("size=", "").replace("\"", "")));
                    } else if p == "&liquidglass}" {
                        css.push_str("  background: rgba(255, 255, 255, 0.15);\n  backdrop-filter: blur(10px);\n  border: 1px solid rgba(255, 255, 255, 0.3);\n  border-radius: 16px;\n  padding: 15px;\n  box-shadow: 0 4px 30px rgba(0,0,0,0.1);\n");
                    } else if p == "&dilute}" { css.push_str("  opacity: 0.7;\n  filter: brightness(1.2);\n"); }
                }
                css.push_str("}\n");
            }
            continue;
        }
        let mut mapped = l.replace("klasse=", "class=");
        mapped = mapped
            .replace("<04", "<title").replace("</04>", "</title>")
            .replace("<09>", "<script src=\"script.js\"></script>")
            .replace("<10", "<div").replace("</10>", "</div>")
            .replace("<12", "<header").replace("</12>", "</header>")
            .replace("<14", "<main").replace("</14>", "</main>")
            .replace("<15", "<section").replace("</15>", "</section>")
            .replace("<20", "<p").replace("</20>", "</p>")
            .replace("<21", "<h1").replace("</21>", "</h1>")
            .replace("<22", "<h2").replace("</22>", "</h2>")
            .replace("<27", "<a").replace("</27>", "</a>")
            .replace("<98", "<marquee").replace("</98>", "</marquee>")
            .replace("<99", "<span style='display:none;'").replace("</99>", "</span>")
            .replace("<100", "<span style='display:inline-block; transform: rotate(180deg);'").replace("</100>", "</span>");
        html.push_str(&format!("  {}\n", mapped));
    }
    (html, css)
}

fn procesar_flamedash(contenido: &str) -> String {
    let mut js = String::new();
    js.push_str("// JavaScript generado por el motor de FLAME\n");
    js.push_str("document.addEventListener('DOMContentLoaded', () => {\n");
    let mut en_dash = false;
    for linea in contenido.lines() {
        let l = linea.trim();
        if l.starts_with("[dash=") { en_dash = true; }
        if en_dash {
            if l.contains("activision~zeit=infinito") {
                js.push_str("    const panel = document.createElement('div');\n");
                js.push_str("    panel.innerHTML = '<h3>Panel Antitrampa Activado</h3>';\n");
                js.push_str("    panel.style.cssText = 'position:fixed; bottom:20px; right:20px; padding:15px; background:#ff4500; color:white; border-radius:10px; box-shadow:0 0 10px rgba(0,0,0,0.5); z-index:9999;';\n");
                js.push_str("    document.body.appendChild(panel);\n");
            }
            if l.contains("actende~zeit=original") {
                js.push_str("    const btn = document.createElement('button');\n");
                js.push_str("    btn.innerText = 'Cerrar';\n");
                js.push_str("    btn.style.cssText = 'margin-top:10px; background:black; color:white; border:none; padding:5px 10px; cursor:pointer;';\n");
                js.push_str("    btn.onclick = () => panel.remove();\n");
                js.push_str("    panel.appendChild(btn);\n");
            }
        }
        if l.ends_with("]") && en_dash { en_dash = false; }
    }
    js.push_str("});\n");
    js
}

fn procesar_flameformel(contenido: &str) -> String {
    let mut reporte = String::new();
    reporte.push_str("--- REPORTE DE EMPAQUETADO (flameformel) ---\n\n");
    
    let mut manifiesto = String::new();
    let mut archivo = String::new();
    let mut linkname = String::new();
    let mut inhalt = String::new();
    let mut aktion = String::new();

    for linea in contenido.lines() {
        let l = linea.trim();
        if l.starts_with("manifiesto=") {
            manifiesto = l.replace("manifiesto=", "").trim().to_string();
        }
        if l.contains("<archiv>") {
            // Correccion: Se agrega .replace(">", "") para limpiar el cierre
            archivo = l.replace("<archiv>", "").replace("\"", "").replace(">", "").trim().to_string();
        }
        if l.contains("[linkname=") {
            linkname = l.replace("[linkname=", "").replace("]", "").replace("\"", "").trim().to_string();
        }
        if l.contains("[Inhalt=") {
            inhalt = l.replace("[Inhalt=", "").replace("]", "").trim().to_string();
        }
        if l.contains("[aktion=") {
            aktion = l.replace("[aktion=", "").replace("]", "").trim().to_string();
        }
    }

    if !archivo.is_empty() {
        reporte.push_str(&format!("Lenguaje Destino: {}\n", manifiesto));
        reporte.push_str(&format!("Archivo a importar: {}\n", archivo));
        reporte.push_str(&format!("Contenido: {}\n", inhalt));
        reporte.push_str(&format!("Accion: {}\n", aktion));
        reporte.push_str(&format!("Enlazado como: {}\n", linkname));
        reporte.push_str("\nPaquete generado con exito. Listo para usarse en FLAME.\n");
    } else {
        reporte.push_str("Error: No se encontro ninguna etiqueta <archiv> en el archivo.\n");
    }
    reporte
}

fn ejecutar_lineas(lineas: &[&str], variables: &mut HashMap<String, String>, funciones: &HashMap<String, Vec<String>>) {
    let mut i = 0; let mut saltar_bloque = false; let mut saltar_siguiente = false;
    while i < lineas.len() {
        let l = lineas[i].trim();
        if l.starts_with("###") || l.is_empty() { i += 1; continue; }
        if l.starts_with("ja ") {
            let cond = l.replace("ja ", "").replace("[", "").trim().to_string();
            let res = evaluar_condicion(&cond, variables);
            saltar_bloque = !res; saltar_siguiente = res; i += 1; continue;
        }
        if l == "] aber [" { saltar_bloque = saltar_siguiente; i += 1; continue; }
        if l == "]" { saltar_bloque = false; i += 1; continue; }
        if saltar_bloque { i += 1; continue; }
        if l.ends_with(":") && !l.starts_with("@") && !l.starts_with("manifest!") {
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
            if p.len() == 2 { variables.insert(p[0].trim().replace("@", ""), evaluar(&p[1].replace(":", ""), variables)); }
        } 
        else if l.starts_with("manifest!") {
            println!("FLAME dice: {}", evaluar(&l.replace("manifest! = ", "").replace(":", ""), variables));
        }
        i += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let archivo_fm = if args.len() > 1 { &args[1] } else { "programa.fm" };

    println!("Motor de FLAME iniciado");
    println!("Leyendo archivo: {}", archivo_fm);
    
    if let Ok(contenido) = fs::read_to_string(archivo_fm) {
        if contenido.contains("!stil nml") {
            println!("Modo NML + StiMoB detectado. Generando HTML y CSS...");
            let (html_out, css_out) = procesar_nml(&contenido);
            fs::write("index.html", html_out).expect("Error creando HTML");
            fs::write("style.css", css_out).expect("Error creando CSS");
            println!("Pagina web y estilos creados.");
            return;
        }

        if contenido.contains("!stil flamedash") {
            println!("Modo flamedash detectado. Generando JavaScript...");
            let js_out = procesar_flamedash(&contenido);
            fs::write("script.js", js_out).expect("Error creando JS");
            println!("Script de JavaScript creado.");
            return;
        }

        if contenido.contains("!stil flameformel") {
            println!("Modo flameformel detectado. Procesando paquete...");
            let reporte = procesar_flameformel(&contenido);
            println!("{}", reporte);
            fs::write("manifiesto.txt", reporte).expect("Error creando manifiesto");
            println!("Reporte guardado en 'manifiesto.txt'.");
            return;
        }

        let mut variables: HashMap<String, String> = HashMap::new();
        let mut funciones: HashMap<String, Vec<String>> = HashMap::new(); 
        let lineas: Vec<&str> = contenido.lines().collect();
        let mut i = 0; let mut en_flamecode = false;
        while i < lineas.len() {
            let l = lineas[i].trim();
            if l == "<flamecode>" { en_flamecode = true; i += 1; continue; }
            if l == "</flamecode>" { en_flamecode = false; i += 1; continue; }
            if l.starts_with("!stil") { i += 1; continue; }
            if !en_flamecode { i += 1; continue; }

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
