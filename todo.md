# Done (Completado)
 -> Agregar una librería para el CLI (Implementado usando `clap`).
 -> Flag principal para descargar un archivo (Implementado con `--file-name` y la conexión básica a Transmission lograda).
 -> Remover impresiones de log de desarrollo.
 -> Asegurar que la solicitud inicial de descarga no arroje errores.
 -> [Bug] Corregir el `time::sleep` en `show_single_download`: le falta el `.await` al final, por lo que el future no se está ejecutando correctamente.
 -> Agregar formato de tabla a la salida de `--list-downloading-files` para que sea más fácil de leer en la terminal.

# Doing (En Progreso)
 -> Integrar las funciones lógicas de `rtorrenty_logic.rs` con las ramas de ejecución en `rtorrenty_cli.rs` (el esqueleto ya está, falta el "cableado").



## Bugs & Refactorización
 -> [Refactor] Eliminar la ruta absoluta hardcodeada del archivo de configuración (`/home/carlinux/rtorrenty/config.toml` en `helpers.rs`) y hacer que se resuelva dinámicamente (ej. `$HOME/.config/rtorrenty/config.toml`).
 -> [Refactor] Mover la URL de Transmission (`http://localhost:9091/...`) y la autenticación al archivo de configuración, por si necesitas usar credenciales o cambia el puerto en el futuro.



## Funcionalidades Principales
 -> Conectar el flag `--list-downloading-files` con la función `show_downloads`.
 -> Conectar el flag `--download-folder` para actualizar/leer la configuración de destino.
 -> Implementar la lógica para el flag `--delete-file` (usar la API de Transmission para remover un torrent dado su ID).
 -> Crear el archivo de configuración por defecto automáticamente en la primera ejecución del CLI (Nota: `cargo install` no ejecuta scripts post-instalación, por lo que es mejor que tu `main.rs` o `helpers.rs` cree el archivo base si no lo encuentra al iniciar).



## Mejoras a Futuro (Nice to have)
 -> Soportar descargas mediante "Magnet Links" (actualmente el código asume que siempre será un archivo `.torrent` local para codificar a base64).
 -> Flags para Pausar (`--pause`) y Reanudar (`--resume`) un torrent.
