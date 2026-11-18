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
```
!stil flamecode
<flamecode>
manifest! = "Hello, World!":

@usuario = eingabe! = "Ingresa tu nombre para comenzar:":
@nivel = "0":
@es_valido = wah:

rolle = saludar_inicio [
    manifest! = "Hola " H @usuario H ", bienvenido al sistema.":
]

saludar_inicio:

ja @es_valido [
    manifest! = "El estado del usuario es valido.":
] aber [
    manifest! = "El estado del usuario es invalido.":
]

manifest! = "Iniciando secuencia de niveles:":

laos = aus="1" ~ bis="3" [
    @nivel = @nivel H "1":
    manifest! = "Subiendo al nivel " H @nivel:
    
    ja @nivel == "2" [
        manifest! = "Alerta: Has llegado al nivel intermedio.":
    ]
]

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

manifest! = "Probando sistema de proteccion:":

versuch [
    @error_calculado = "10" T "0":
    manifest! = "Esta linea no deberia imprimirse.":
] fang (fehler) [
    manifest! = "Error controlado: " H fehler:
]

manifest! = "--- SISTEMA FLAME FINALIZADO CON EXITO ---":
</flamecode>
```
## 1. FLAME Core (`flamecode`)

El modo `flamedash` es un modo de Flame diferente que flamecode habla en lenguaje de Flame Pero este es modo no hace lo mismo que flamecode hace otra cosa distinta
este te da atajós para no escribir código largo de otro lenguaje 
usando las librerías de flamelibrarys ejecuta

**una explicación menos anbigua**
imagínate que quieres una función de antitrampas de un panel antitrampas pero hecho con código de Java que eso sería gigantesco un código gigante pero venís y utilizas flamedash tú escribes a flin Dash lo que quieres obviamente usando lenguaje de flame y sabiendo su estructura le pides que te haga eso Y en vez de escribir el código gigante de Java solo escribo un código corto de flame y este te dará lo mismo que es Java utilizando la flamelibrarys A flamejavalibrarys siempre les ofrecemos códigos actualizados de Nueva generación

**EJEMPLO CODIGO FLAMEDASH** 
```
!stil flamedash
manifiesto=javascript
<flamedash>
 <javascript>
  [dash=nombre_del_panel
   respektieren=tecladonativo+barradenotificacion.
   wert=tiempo
   activision~zeit=infinito
   actionscript~zeit=inmediato
   actende~zeit=original
  ]
 </javascript>
</flamedash>
```

.

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
