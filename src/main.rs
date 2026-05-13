mod service;
mod utils;
mod models;
mod controller;
mod config;
mod repository;

//cambiar por los controllers que tengas
use controller::vuelo_controller::vuelo_router;
use controller::pasajero_controller::pasajero_router;

use config::config::crear_pool;

#[tokio::main]
async fn main() {
    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor escuchando en http://{direccion}");

    let pool = crear_pool()
        .await
        .expect("No se pudo conectar a la base de datos");

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al iniciar el servidor");
}



fn unificar_routers(pool: sqlx::PgPool) -> axum::Router {
    let mut router3 = vuelo_router(pool.clone());
    let router4= pasajero_router(pool.clone());
    router3.merge(router4)
}