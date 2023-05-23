> This repo is no longer mantained nor updated and we are not pretending to do so.

# API publica de eventos - dotlabs()

Bienvenidx a la API pública de eventos de dotlabs().

Estamos tratando de proporcionar una forma sencilla de interactuar con nuestros datos y estamos abiertos a sugerencias. Utilízalo para practicar, para divertirte o para lo que quieras, **pero por favor considera el consumo que generas**.

> Considera contribuir. Tu ayuda es indispensable.

## Endpoints

*Próximamente publicaremos algunos endpoints para nuestros eventos semanales*

## Crea tu propia API

Si quieres usar este proyecto como base para el tuyo sigue los pasos de abajo:

### Prerrequisitos

Necesitaras tener instalado lo siguiente en tu máquina:

- [Rust](https://www.rust-lang.org/tools/install)
- [MongoDB Account](https://www.mongodb.com/cloud/atlas)

### Configuración

Crea un archivo `.env` dentro de `/server`. Este archivo contendrá como minimo lo siguiente:

```bash
MONGO_DB_URI=<your_mongo_db_uri>
```

### Intalando las dependencias

Dentro del directorio `/server` ejecuta el comando:

```bash
cargo build
```

## Running

Dentro del directorio `/server` ejecuta el comando:

```bash
cargo run
```

Por defecto, el servidor se ejecutará en el puerto 8080 y el host será el 127.0.0.1. Puedes cambiar esto configurando la variable de entorno `PORT` y `HOST` respectivamente.
