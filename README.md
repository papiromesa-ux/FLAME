# FLAME MODELO 0.0.1

# MISIÓN 
la misión del software FLAME es hacer que la programación sea más fácil,ligera,versátil usando métodos de programación especiales y únicas.

# ENHORABUENA 
FLAME viene de la palabra en español flama que su intención es expresar que es potente como una flama de fuego y ligera de pesor como ella 

# MOTOR 
El motor de Flame está construido de Rust.

# SINTAXIS BÁSICA 
La sintaxis de FLAME se caracteriza por la mezcla del español, alemán e inglés, para mayor versátilidad se mezcla los tres lenguajes.

**Bloques de codigo**
usa corchetes `[]`
apertura `[`
cierre `]`

**Cierre de instrucciones**
usa `:`

**comentario**
usa `###comentario de patapim###`

**conjugado de relación**



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
