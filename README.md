# FLAME Programming Language

FLAME es un ecosistema de programación de alto rendimiento y propósito general, inspirado en la rigurosidad de Rust, la estructura de Java y la versatilidad de los lenguajes modernos. 

El núcleo del lenguaje está construido desde cero en Rust, lo que garantiza un manejo de memoria seguro y una velocidad de ejecución excepcional. FLAME no es solo un lenguaje, sino un ecosistema compuesto por múltiples modos de uso, diseñados para cubrir la lógica, la estructura web, los estilos visuales, la interactividad y la gestión de dependencias.

La sintaxis de FLAME se caracteriza por la mezcla del español, alemán e inglés, eliminando caracteres especiales (como diéresis o la letra eszett) para evitar problemas de codificación y garantizar que el motor lo lea sin errores.

## Reglas Universales de Sintaxis

Antes de explorar los modos, es fundamental comprender las reglas base del ecosistema:

1.  **Bloques de código:** Se utilizan corchetes `[ ]` en lugar de llaves `{ }` para delimitar bloques lógicos.
2.  **Cierre de instrucciones:** Toda instrucción o línea de código termina con dos puntos `:` (no se utiliza punto y coma).
3.  **Comentarios:** Se encierran entre tres almohadillas de apertura y cierre: `### Comentario ###`
4.  **Relación de tiempo:** La virguilla `~` se utiliza para unir acciones con contextos temporales (ejemplo: `actionscript~zeit`).
5.  **Declaración de modo:** Todo archivo `.fm` debe iniciar con la etiqueta `!stil` seguida del modo correspondiente.

---

## 1. FLAME Core (`flamecode`)

El modo `flamecode` es el cerebro lógico y nativo del ecosistema. No requiere de otros lenguajes para funcionar. Se utiliza para manipular datos, ejecutar algoritmos, tomar decisiones y procesar lógica compleja.

### Sintaxis y Características

*   **Variables:** Se declaran utilizando el símbolo `@` antes del nombre.
    ```flame
    @nombre = "Creador":
    @edad = "25":
    ```
*   **Impresión:** Se utiliza la palabra reservada `manifest!` seguida de un signo igual.
    ```flame
    manifest! = "Hola mundo":
    ```
*   **Entrada de datos:** Utiliza `eingabe!` para detener el motor y solicitar información al usuario.
    ```flame
    @respuesta = eingabe! = "¿Cómo te llamas?":
    ```
*   **Matemáticas y Operadores:** FLAME no utiliza los símbolos tradicionales (+, -, *, /). Utiliza letras:
    *   `H` = Suma (o concatenación de textos)
    *   `S` = Resta
    *   `M` = Multiplicación
    *   `T` = División
    *   `R` = Módulo
    *   `P` = Potencia
    ```flame
    @resultado = @numero1 H @numero2:
    ```
*   **Lógica Booleana:** Usa `wahr` (verdadero) y `falsch` (falso). Los operadores lógicos son `und` (AND), `oder` (OR), `nicht` (NOT).

### Control de Flujo y Estructuras

*   **Condicionales (`ja` / `aber`):** Evalúa condiciones con operadores como `==`, `!=`, `>`, `<`.
    ```flame
    ja @edad >= "18" [
        manifest! = "Mayor de edad":
    ] aber [
        manifest! = "Menor de edad":
    ]
    ```
*   **Bucles (`laos`):** Itera desde un punto de inicio (`aus`) hasta un fin (`bis`).
    ```flame
    laos = aus="1" ~ bis="5" [
        manifest! = "Repitiendo ciclo":
    ]
    ```
*   **Funciones (`rolle`):** Agrupa código reutilizable. El retorno se hace con `zurueck`.
    ```flame
    rolle = sumar [
        zurueck = "5" H "5":
    ]
    @mi_resultado = sumar:
    ```
*   **Manejo de Errores (`versuch` / `fang`):** Evita que el motor colapse ante errores en tiempo de ejecución.
    ```flame
    versuch [
        @resultado = "10" T "0":
    ] fang (fehler) [
        manifest! = "Error detectado: " H fehler:
    ]
    ```

---

## 2. NML (Numeric Markup Language)

NML es el lenguaje de marcado estructural del ecosistema, diseñado como reemplazo de HTML. Su principal característica es que no utiliza etiquetas de texto, sino números de dos cifras agrupados por familias lógicas, lo que reduce drásticamente la cantidad de código y acelera el procesamiento del motor.

### Familias de Etiquetas NML

*   **Familia 00 (Documento):** 01 (html), 02 (head), 03 (body), 04 (title), 05 (meta), 09 (script).
*   **Familia 10 (Estructura):** 10 (div), 12 (header), 13 (footer), 14 (main), 15 (section).
*   **Familia 20 (Texto):** 20 (p), 21 (h1), 22 (h2), 27 (a).
*   **Familia 70 (Tablas):** 70 (table), 75 (tr), 77 (td).

### Etiquetas Especiales NML
*   `98`: Texto rotativo.
*   `99`: Texto invisible (solo visible para el sistema o buscadores).
*   `100`: Texto transformado (rotación de 180 grados).

### Ejemplo de NML

```flame
!stil nml
<01>
  <02>
    <04>Mi Página FLAME</04>
  </02>
  <03>
    <12>
      <21>Bienvenido a NML</21>
    </12>
    <14>
      <20>Este es un párrafo generado con el motor numérico.</20>
    </14>
  </03>
</01>
