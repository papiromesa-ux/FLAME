use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub fn ejecutar_flame_web(codigo_fm: &str) -> String {
    let mut output = String::new();
    let mut variables: HashMap<String, String> = HashMap::new();
    
    output.push_str("Motor de FLAME iniciado en la Web.\n");

    for linea in codigo_fm.lines() {
        let l = linea.trim();
        if l.is_empty() || l.starts_with("###") || l.starts_with("!stil") || l.starts_with("<flamecode>") || l.starts_with("</flamecode>") {
            continue;
        }

        if l.starts_with("@") {
            let p: Vec<&str> = l.split('=').collect();
            if p.len() == 2 {
                let nombre = p[0].trim().replace("@", "");
                let valor = p[1].replace(":", "").replace("\"", "").trim().to_string();
                variables.insert(nombre, valor);
            }
        }
        else if l.starts_with("manifest!") {
            let expr = l.replace("manifest! = ", "").replace(":", "").trim().to_string();
            if expr.starts_with("@") {
                let var = expr.replace("@", "");
                if let Some(val) = variables.get(&var) {
                    output.push_str(&format!("FLAME dice: {}\n", val));
                }
            } else {
                output.push_str(&format!("FLAME dice: {}\n", expr.replace("\"", "")));
            }
        }
    }
    output
}
