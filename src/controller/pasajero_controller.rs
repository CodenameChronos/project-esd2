use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use crate::service::pasajero_service::{
    obtener_pasajeros,
    crear_pasajero,
    actualizar_pasajero,
    actualizar_pasajero_por_id,
    eliminar_pasajero,
    eliminar_pasajero_por_id,
};

pub fn pasajero_router(pool: PgPool) -> Router {
    Router::new()
        .route("pasajeros", get(obtener_pasajeros))
        .route("pasajeros", post(crear_pasajero))
        .route("pasajeros/:id_pasajero", put(actualizar_pasajero))
        .route("pasajeros/:id_pasajero", delete(eliminar_pasajero))
        .route("pasajeros/:id_pasajero", put(actualizar_pasajero_por_id))
        .route("pasajeros/:id_pasajero", delete(eliminar_pasajero_por_id))
        .with_state(pool)
}