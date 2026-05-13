use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use crate::service::vuelo_service::{
    obtener_vuelos,
    crear_vuelo,
    actualizar_vuelo,
    actualizar_vuelo_por_id,
    eliminar_vuelo,
    eliminar_vuelo_por_id,
};

pub fn vuelo_router(pool: PgPool) -> Router {
    Router::new()
        .route("vuelos", get(obtener_vuelos))
        .route("vuelos", post(crear_vuelo))
        .route("vuelos/:id_vuelo", put(actualizar_vuelo))
        .route("vuelos/:id_vuelo", delete(eliminar_vuelo))
        .route("vuelos/:id_vuelo", put(actualizar_vuelo_por_id))
        .route("vuelos/:id_vuelo", delete(eliminar_vuelo_por_id))
        .with_state(pool)
}