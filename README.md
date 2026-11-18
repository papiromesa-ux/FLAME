# FLAME MODELO 0.0.1

# MISIÓN 
la misión del software FLAME es hacer que la programación sea más fácil,ligera,versátil usando métodos de programación especiales y únicas.

# ENHORABUENA 
FLAME viene de la palabra en español flama que su intención es expresar que es potente como una flama de fuego y ligera de pesor como ella.

FLAME está inspirado en Rust como el 70%  y en c++ en un 15% e también en Java en 5% y de último y menos importante en 10% la mezcla de javascript y Python 

# MOTOR 
El motor de Flame está construido de Rust por su potencia y para lo que queremos para Flame es un perfecto lenguaje para la creación del motor también se tuvo que hacer varios usos de otros lenguajes como de c++ y de assebly para su arranque pero lo el que hizo mayor trabajo el trabajo pesado fue Rust.

# SINTAXIS BÁSICA 
La sintaxis de FLAME es parecida a la de rust pero se caracteriza por la mezcla del español, alemán e inglés, para mayor versátilidad se mezcla los tres lenguajes.

**Bloques de codigo**
usa corchetes `[]`
apertura `[`
cierre `]`

**Cierre de instrucciones**
usa `:`

**comentario**
usa `###comentario de patapim###`

**conjugado de relación**
cómo Flame se inspiran en Rust es muy estricto con la memoria y te pide que programes el tiempo pero no solo por la memoria solo es una de ellas también es para saber qué relación a cada comando para ejecutar su acción de proceso les enseñare un uso de el elemento `~` cómo ejemplo les enseñare este comando `actionscript~zeit` en lenguaje de flame entiende que la acción de atajó de lógica se le modificara el tiempo de acción digamos 5.s que sucede en sí es lo siguiente 

imagínate que Flame hablé como un humano y piense como un humano y haces el comando 

`actionscript` 

Flame dice = okey el usuario puso `actionscript` le va a cambiar a la acción de la función de lógica pero el que 

`~zeit` 

Flame dice = ya entendí le quiere cambiar el tiempo de acción de la función de lógica cuanto tiempo le vas a cambiar?

`=3.s`

Flame dice = okey cambio del tiempo de acción de la funccion de lógica a 3 segundos entonces modificacion de tiempo acción de la función de lógica está consedido con éxito ya no utilizare el de predeterminado sino de tiempo que has puesto tú 

**la declaración del modo de uso de Flame**
un archivo `fm` debe iniciar con su declaración de estilo `!stil` seguida del modo

*flamedash*

*flamecode*

*flameformel*

así Flame entendera que vas hacer con Flame 

---

## 1. FLAME Core (`flamecode`)

El modo `flamecode` es el cerebro lógico y nativo del software. todo el software habla lenguaje de Flame para funcionar. Se utiliza para escribir código pero con lenguaje de Flame en vez que con javascript o otros lenguajes.

**EJEMPLO CODIGO FLAMECODE**
`!stil flamecode
<flamecode>
### ==========================================
### CODIGO DE PRUEBA COMPLETO DE FLAMECODE
### ==========================================

manifest! = "--- INICIANDO SISTEMA FLAME ---":

### 1. ENTRADA DE DATOS Y VARIABLES
@usuario = eingabe! = "Ingresa tu nombre para comenzar:":
@nivel = "0":
@es_valido = wahr:

### 2. FUNCIONES (rolle)
rolle = saludar_inicio [
    manifest! = "Hola " H @usuario H ", bienvenido al sistema.":
]

### Llamada a la funcion
saludar_inicio:

### 3. CONDICIONALES Y BOOLEANOS (ja / aber)
ja @es_valido [
    manifest! = "El estado del usuario es valido (wahr).":
] aber [
    manifest! = "El estado del usuario es invalido (falsch).":
]

### 4. BUCLES (laos)
manifest! = "Iniciando secuencia de niveles:":
laos = aus="1" ~ bis="3" [
    @nivel = @nivel H "1":
    manifest! = "Subiendo al nivel " H @nivel:
    
    ja @nivel == "2" [
        manifest! = "Alerta: Has llegado al nivel intermedio.":
    ]
]

### 5. MATEMATICAS Y CONCATENACION
@num_a = "10":
@num_b = "5":
@suma = @num_a H @num_b:
@resta = @num_a S @num_b:
@multiplicacion = @num_a M @num_b:
@division = @num_a T @num_b:

manifest! = "Resultados matematicos:":
manifest! = "Suma: " H @suma:
manifest! = "Resta: " H @resta:
manifest! = "Multiplicacion: " H @multiplicacion:
manifest! = "Division: " H @division:

### 6. MANEJO DE ERRORES (versuch / fang)
manifest! = "Probando sistema de proteccion contra errores:":
versuch [
    ### Intentamos una division por cero a proposito ###
    @error_calculado = "10" T "0":
    manifest! = "Esta linea no deberia imprimirse si hay error.":
] fang (fehler) [
    manifest! = "Error controlado por el motor: " H fehler:
]

manifest! = "--- SISTEMA FLAME FINALIZADO CON EXITO ---":

</flamecode>`

## 2. FLAME Core (`flamedash`)

El modo `flamedash` es un modo de flame que habla lenguaje de Flame y se utiliza para atajós para no escribir códigos gigantes 
**EJEMPLO CODIGO FLAMEDASH**

## 2. FLAME Core (`flameformel`)

El modo `flameformel` es un modo de flame que habla lenguaje de Flame y se utiliza para crear tus propios flamedash 
**EJEMPLO CODIGO FLAMEFORMEL**


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
