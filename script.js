// 🔥 JavaScript generado por el motor de FLAME 🔥
document.addEventListener('DOMContentLoaded', () => {
    console.log('FLAME flamedash: Inicializando panel...');
    // Respetando teclado nativo y barra de notificaciones
    console.log('Sistema respetado: no se bloqueará la interfaz.');
    // Panel configurado para tiempo infinito
    const panel = document.createElement('div');
    panel.innerHTML = '<h3>Panel Antitrampa Activado</h3>';
    panel.style.cssText = 'position:fixed; bottom:20px; right:20px; padding:15px; background:#ff4500; color:white; border-radius:10px; box-shadow:0 0 10px rgba(0,0,0,0.5); z-index:9999;';
    document.body.appendChild(panel);
    // Ejecución inmediata del script
    // Cierre manejado por botón original
    const btn = document.createElement('button');
    btn.innerText = 'Cerrar';
    btn.style.cssText = 'margin-top:10px; background:black; color:white; border:none; padding:5px 10px; cursor:pointer;';
    btn.onclick = () => panel.remove();
    panel.appendChild(btn);
});
