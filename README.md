# 🔥 FLAME RELEASE 0.1

**FLAME** es un lenguaje de programación independiente y de alto rendimiento, inspirado en la rigurosidad de Rust, la estructura de Java y la claridad de Python. 

Su identidad se basa en una mezcla única del **español, alemán e inglés**, eliminando caracteres extraños (como ñ, tildes, diéresis o ß) para garantizar que el motor lo lea sin errores. 

El ecosistema FLAME está compuesto por tres lenguajes interconectados que trabajan en un solo motor (construido desde cero en **Rust y C++**). La extensión oficial de sus archivos es **`.fm`**.

---

## 🛠️ Reglas Universales de FLAME

1. **Bloques:** Se abren y cierran con corchetes `[ ]` (inspirado en Rust, pero sin usar llaves `{ }`).
2. **Cierre de línea:** Toda instrucción termina con dos puntos `:` (no se usa `;`).
3. **Comentarios:** Se escriben encerrados entre almohadillas `### Comentario ###`.
4. **Relación:** La virguilla `~` une propiedades o acciones con su contexto (ej. `actionscript~zeit`).
5. **Comillas:** Los textos y números matemáticos en el código lógico van entre `" "`. En el modo de marcado (NML) no se usan comillas.

---

## 🧩 1. FLAME Core (`!stil flamecode`)
Es el cerebro lógico y nativo del lenguaje. No necesita conectarse a otros lenguajes.

*   **Variables:** Se declaran con `@`. Ej: `@vidas = "3":`
*   **Imprimir:** `manifest! = "Hola mundo":`
*   **Entrada de datos:** `@dato = eingabe! = "Pregunta":`
*   **Matemáticas (Letras):** `H` (sumar), `S` (restar), `M` (multiplicar), `T` (dividir).
*   **Condiciones:** `ja` (si) / `aber` (sino). Se usan símbolos lógicos (`==`, `!=`, `>`, `<`).
*   **Booleanos:** `wahr` (verdadero), `falsch` (falso).
*   **Bucles (`laos`):** Usa límites lógicos (`aus="1" ~ bis="5"`) y de ejecución (`eingang="1" ~ ausgang="5"`).
*   **Funciones:** `rolle = nombre_funcion[ ... ]` (ej. `rolle = matematicas[ ... ]`).

**Ejemplo de código:**
```flame
!stil flamecode
<flamecode>
@numero1 = "10":
@numero2 = "5":
@suma = @numero1 H @numero2:
manifest! = "El resultado es " H @suma:
</flamecode>
