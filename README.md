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
## 1. FLAME Core (`flamedash`)

El modo `flamedash` es un modo de Flame diferente que flamecode habla en lenguaje de Flame Pero este es modo no hace lo mismo que flamecode hace otra cosa distinta
este te da atajós para no escribir código largo de otro lenguaje 
usando las librerías de flamelibrarys ejecuta

**una explicación menos anbigua**
imagínate que quieres una función de antitrampas de un panel antitrampas pero hecho con código de Java que eso sería gigantesco un código gigante pero venís y utilizas flamedash tú escribes a flin Dash lo que quieres obviamente usando lenguaje de flame y sabiendo su estructura le pides que te haga eso Y en vez de escribir el código gigante de Java solo escribo un código corto de flame y este te dará lo mismo que es Java utilizando la flamelibrarys A flamejavalibrarys siempre les ofrecemos códigos actualizados de Nueva generación

**EJEMPLO CODIGO FLAMEDASH** 
javascriptpanelatajo.fm
```
!stil flamedash
manifiesto=javascript
<flamedash>
 <javascript>
<archiv>=panelantitrampahecho100%pormínohehechoconflame.js
  [dash=panelantitrampa
   respektieren=tecladonativo+barradenotificacion.
   wert=tiempo
   activision~zeit=infinito
   actionscript~zeit=inmediato
   actende~zeit=original=
$=javascript 
  </archiv>=panelantitrampahecho100%pormínohehechoconflame.js
 </javascript>
</flamedash>
```
panelantitrampahecho100%pormínohehechoconflame.js
```
(function () {
  // 
  if (window.__PANEL_ANTITRAMPA_INICIADO__) return;
  window.__PANEL_ANTITRAMPA_INICIADO__ = true;

  let panelBloqueado = false;
  let tecladoAbierto = false;
  let alturaBase = window.visualViewport ? window.visualViewport.height : window.innerHeight;
  let hostElemento = null;
  let motivoNodo = null;

  // 1. ASEGURAR VIEWPORT PARA TECLADO Y BARRA DE NOTIFICACIONES
  function configurarViewport() {
    let meta = document.querySelector('meta[name="viewport"]');
    if (!meta) {
      meta = document.createElement('meta');
      meta.name = 'viewport';
      document.head.appendChild(meta);
    }
    const contenido = meta.getAttribute('content') || 'width=device-width, initial-scale=1.0';
    if (!contenido.includes('viewport-fit=cover')) {
      meta.setAttribute('content', `${contenido}, viewport-fit=cover, interactive-widget=resizes-content`);
    }
  }

  // 2. GUARDADO UNIVERSAL DE PROGRESO (ADAPTABLE A CUALQUIER HTML)
  function guardarTodoElProgreso() {
    const campos = document.querySelectorAll('input, textarea, select');
    const datos = {};

    campos.forEach((campo, index) => {
      const clave = campo.id || campo.name || `campo_auto_${index}`;
      if (campo.type === 'checkbox' || campo.type === 'radio') {
        datos[clave] = campo.checked;
      } else if (campo.type !== 'password') {
        datos[clave] = campo.value;
      }
    });

    try {
      localStorage.setItem('antitrampa_progreso_global', JSON.stringify(datos));
      localStorage.setItem('antitrampa_ultima_ruta', window.location.href);
    } catch (e) {
      // Ignorar si el almacenamiento está restringido
    }
  }

  function restaurarTodoElProgreso() {
    try {
      const guardado = localStorage.getItem('antitrampa_progreso_global');
      if (!guardado) return;
      const datos = JSON.parse(guardado);
      const campos = document.querySelectorAll('input, textarea, select');

      campos.forEach((campo, index) => {
        const clave = campo.id || campo.name || `campo_auto_${index}`;
        if (datos[clave] !== undefined) {
          if (campo.type === 'checkbox' || campo.type === 'radio') {
            campo.checked = datos[clave];
          } else {
            campo.value = datos[clave];
          }
        }
      });
    } catch (e) {}
  }

  // 3. CONSTRUIR EL PANEL AISLADO EN SHADOW DOM (INMUNE A CUALQUIER CSS EXTERNO)
  function crearPanelAislado() {
    if (hostElemento) return;

    hostElemento = document.createElement('div');
    hostElemento.id = 'sistema-antitrampa-aislado';
    // Máximo z-index posible en navegadores para estar encima de cualquier CSS
    hostElemento.style.cssText = 'position: fixed !important; inset: 0 !important; z-index: 2147483647 !important; display: none;';

    const shadow = hostElemento.attachShadow({ mode: 'closed' });

    const estiloInterno = document.createElement('style');
    estiloInterno.textContent = `
      :host {
        all: initial;
        font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      }
      .overlay {
        position: fixed;
        inset: 0;
        width: 100vw;
        height: 100dvh;
        background: rgba(12, 12, 16, 0.97);
        backdrop-filter: blur(8px);
        -webkit-backdrop-filter: blur(8px);
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        box-sizing: border-box;
        /* Respeta barra de notificaciones, notch y bordes nativos */
        padding-top: max(20px, env(safe-area-inset-top));
        padding-bottom: max(20px, env(safe-area-inset-bottom));
        padding-left: max(20px, env(safe-area-inset-left));
        padding-right: max(20px, env(safe-area-inset-right));
        color: #ffffff;
        text-align: center;
        user-select: none;
      }
      .tarjeta {
        background: #1b1b24;
        border: 2px solid #ff3b3b;
        border-radius: 16px;
        padding: 28px 22px;
        width: 100%;
        max-width: 380px;
        box-shadow: 0 20px 50px rgba(0, 0, 0, 0.85);
        box-sizing: border-box;
      }
      h2 {
        margin: 0 0 12px 0;
        font-size: 20px;
        color: #ff4d4d;
        letter-spacing: 0.3px;
      }
      p {
        margin: 0 0 14px 0;
        font-size: 15px;
        line-height: 1.45;
        color: #e0e0e6;
      }
      .estado-guardado {
        font-size: 13px;
        color: #4ade80;
        margin-bottom: 22px;
      }
      button {
        width: 100%;
        padding: 14px 20px;
        font-size: 16px;
        font-weight: 700;
        color: #ffffff;
        background: #ff3b3b;
        border: none;
        border-radius: 10px;
        cursor: pointer;
        transition: background 0.15s ease;
      }
      button:active {
        background: #c92424;
        transform: scale(0.98);
      }
    `;

    const overlay = document.createElement('div');
    overlay.className = 'overlay';

    const tarjeta = document.createElement('div');
    tarjeta.className = 'tarjeta';

    const titulo = document.createElement('h2');
    titulo.textContent = ' Bloqueo Antitrampa';

    motivoNodo = document.createElement('p');
    motivoNodo.textContent = 'Se detectó salida de la aplicación o superposición.';

    const infoGuardado = document.createElement('div');
    infoGuardado.className = 'estado-guardado';
    infoGuardado.textContent = '✓ Todo tu progreso ha sido guardado.';

    const botonInicio = document.createElement('button');
    botonInicio.textContent = 'Regresar al inicio';

    // ÚNICA FORMA DE CERRAR EL PANEL
    botonInicio.addEventListener('click', () => {
      guardarTodoElProgreso();
      panelBloqueado = false;
      hostElemento.style.display = 'none';
      window.scrollTo({ top: 0, behavior: 'instant' });
      // Si deseas que recargue la página al inicio manteniendo el progreso, descomenta la siguiente línea:
      // window.location.reload();
    });

    tarjeta.append(titulo, motivoNodo, infoGuardado, botonInicio);
    overlay.appendChild(tarjeta);
    shadow.append(estiloInterno, overlay);
    document.documentElement.appendChild(hostElemento);
  }

  // 4. ACTIVACIÓN INMEDIATA E INFINITA
  function activarPanelInmediato(razon) {
    if (panelBloqueado) return;
    guardarTodoElProgreso();
    crearPanelAislado();

    panelBloqueado = true;
    motivoNodo.textContent = razon;
    hostElemento.style.display = 'block';

    if (document.activeElement && typeof document.activeElement.blur === 'function') {
      document.activeElement.blur();
    }
  }

  // 5. DETECCIÓN DE SALIDA, CAMBIO DE APP O VENTANAS ENCIMA
  document.addEventListener('visibilitychange', () => {
    if (document.hidden) {
      activarPanelInmediato('Saliste de la aplicación o cambiaste de pestaña.');
    }
  });

  window.addEventListener('blur', () => {
    setTimeout(() => {
      if (!tecladoAbierto && !document.hasFocus()) {
        activarPanelInmediato('Activaste una ventana o aplicación encima de esta pantalla.');
      }
    }, 80);
  });

  // 6. RESPETO AL TECLADO NATIVO DEL CELULAR
  if (window.visualViewport) {
    window.visualViewport.addEventListener('resize', () => {
      const alturaActual = window.visualViewport.height;
      if (alturaActual < alturaBase * 0.75) {
        tecladoAbierto = true;
      } else {
        tecladoAbierto = false;
        alturaBase = alturaActual;
      }
      if (hostElemento && panelBloqueado) {
        hostElemento.style.height = `${alturaActual}px`;
      }
    });
  }

  // 7. BLOQUEAR BOTÓN "ATRÁS" DEL CELULAR MIENTRAS EL PANEL ESTÁ ACTIVO
  history.pushState(null, '', location.href);
  window.addEventListener('popstate', () => {
    if (panelBloqueado) {
      history.pushState(null, '', location.href);
    }
  });

  // 8. OBSERVADOR DE CAMBIOS EN EL DOM (ADAPTABLE A CUALQUIER MODIFICACIÓN DINÁMICA)
  const observadorDOM = new MutationObserver(() => {
    // Si alguien intenta borrar el panel desde el inspector o mediante otro script, lo vuelve a crear al instante
    if (panelBloqueado && !document.documentElement.contains(hostElemento)) {
      hostElemento = null;
      panelBloqueado = false;
      activarPanelInmediato('Intento de manipulación detectado.');
    }
  });

  // INICIALIZACIÓN AUTOMÁTICA
  function iniciar() {
    configurarViewport();
    restaurarTodoElProgreso();
    document.addEventListener('input', guardarTodoElProgreso, true);
    document.addEventListener('change', guardarTodoElProgreso, true);
    observadorDOM.observe(document.documentElement, { childList: true, subtree: true });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', iniciar);
  } else {
    iniciar();
  }
})();
```
lo que pasa en Flame es un proceso ejemplo como lo siguiente 

*vivo*
javascriptpanelatajo.fm
Flame despierta y piensa okey que haré hoy 

`!stil flamedash`

Flame dice = ahhh con que atajós de que lenguaje 

`manifiesto=javascript`

Flame dice = okey serán Dash de javascript okey entonces solo librerías de javascript pon la etiqueta `<flamedash>` para que puedas poner otras cosas en el mismo archivo y también `<javascript>` para cuándo querrás poner otro Flamedash pero de otro lenguaje te recomiendo usarlo pero si no lo puedes usar está bien no pasa nada 

`[dash=panelantitrampa`

Flame dice = Ah con qué un panelantitrampa 
Okay solo busco en las librerías de javascript el panelantitrampa

Flame busca =
|
flamelibrarys 
|
flamedashlibrarys
|
javascriptlibrarys 
|
paneleslibrarys 
|
panelantitrampalibrarys
|
panelantitrampa-original 

Flame dice = listo ya lo halle

`respektieren=tecladonativo+barradenotificacion`

Flame dice = Ah vas a modificar el javascript del panel anti trampa entonces lo querés que respete el teclado nativo del celular móvil y también la barra de notificaciones del celular móvil que nos active si aprieta esas dos cosas 

Flame busca
panelantitrampalibrarys 
|
panelantitrampa-respetando-tacladonativo+barradenotificaciones 

Flame dice = listo ya encontré el que quieres

`wert=tiempo`

Flame dice= bien el tiempo le quieres cambiar Dios santo ahora que le quiero cambiar de tiempo 

   `activision~zeit=infinito
   actionscript~zeit=inmediato
   actende~zeit=original`

Flame dice = primera modificación el tiempo de visión significa el tiempo que va a estar activo el panel dice que va a ser infinito okay 

hora del tiempo de que el panel va a aparecer digamos cuando haga algo que activa el panel de trampas dice que tiene que ser inmediato 

Y dice De que la forma para que desaparezca el panel anti trampa dice que va a ser del original de original no hay tiempo sino que tenés que apretar un botón que te regrese desde el inicio de nuevo y te borra todo lo que has progresado porque piensa de que hiciste trampa en esa parte 

Flame ejecuta= Pero estas tres modificaciones de más de tiempo no necesito buscar en la librería sino que la puedo modificar yo ponerle de que reaccione que nunca se quita el panel y también de que reacción inmediatamente el tiempo de espera y también que sea como el de la librería original que es de que solo aparezca cuando haga la trampa 

Flame dice = ya hice el cambio 

`]
  </archiv>panelantitrampahecho100%pormínohehechoconflame.js
 </javascript>
</flamedash>`



Flame dice= vaya todo el código de javascript se lo voy a sobreescribir en ese archivo
.
## 1. FLAME Core (`flameformel`)

el modo flameformel es un modo que viene de Flamedash 

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
