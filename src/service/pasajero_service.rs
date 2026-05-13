use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::pasajero::{Pasajero, NuevoPasajero, ActualizarPasajero};
use crate::repository::pasajero_repository;

pub async fn obtener_pasajeros(State(pool): State<PgPool>) -> Json<Vec<crate::models::pasajero::Pasajero>> {
    let pasajeros = pasajero_repository::new(pool);
    match pasajeros.obtener_pasajeros().await {
        Ok(pasajeros) => Json(pasajeros),
        Err(_) => Json(vec![]), 
    }
}

pub async fn crear_pasajero(State(pool): State<PgPool>, Json(nuevo_pasajero): Json<NuevoPasajero>) -> Json<Option<Pasajero>> {
    let pasajeros = pasajero_repository::new(pool);
    match pasajeros.crear_pasajero(nuevo_pasajero).await {
        Ok(pasajero) => Json(Some(pasajero)),
        Err(_) => Json(Pasajero {
            id_pasajero: 0,
            nombre: "Error al crear pasajero".to_string(),
            pasaporte: "Error al crear pasajero".to_string(),
            nacionalidad: "Error al crear pasajero".to_string(),
        }),
    }
}

pub async fn actualizar_pasajero(State(pool): State<PgPool>, Json(pasajero_actualizado): Json<crate::models::pasajero::Pasajero>) -> Json<crate::models::pasajero::Pasajero> {
    let pasajeros = pasajero_repository::new(pool);
    let id = pasajero_actualizado.id_pasajero;
    let nuevo_pasajero = NuevoPasajero {
        nombre: pasajero_actualizado.nombre,
        pasaporte: pasajero_actualizado.pasaporte,
        nacionalidad: pasajero_actualizado.nacionalidad,
    };
    match pasajeros.actualizar_pasajero(id, nuevo_pasajero).await {
        Ok(pasajero) => Json(pasajero),
        Err(_) => Json(Pasajero {
            id_pasajero: 0,
            nombre: "Error al actualizar pasajero".to_string(),
            pasaporte: "Error al actualizar pasajero".to_string(),
            nacionalidad: "Error al actualizar pasajero".to_string(),
        }), 
    }
}

pub async fn actualizar_pasajero_por_id(State(pool): State<PgPool>, Path(id_pasajero): Path<i32>, Json(pasajero_actualizado): Json<ActualizarPasajero>) -> Json<crate::models::pasajero::Pasajero> {
    let pasajeros = pasajero_repository::new(pool);
    let nuevo_pasajero = NuevoPasajero {
        nombre: pasajero_actualizado.nombre,
        pasaporte: pasajero_actualizado.pasaporte,
        nacionalidad: pasajero_actualizado.nacionalidad,
    };
    match pasajeros.actualizar_pasajero(id_pasajero, nuevo_pasajero).await {
        Ok(pasajero) => Json(pasajero),
        Err(_) => Json(Pasajero {
            id_pasajero: 0,
            nombre: "Error al actualizar pasajero".to_string(),
            pasaporte: "Error al actualizar pasajero".to_string(),
            nacionalidad: "Error al actualizar pasajero".to_string(),
        }), 
    }
}

pub async fn eliminar_pasajero(State(pool): State<PgPool>, Path(id_pasajero): Path<i32>) -> Json<bool> {
    let pasajeros = pasajero_repository::new(pool);
    match pasajeros.eliminar_pasajero(id_pasajero).await {
        Ok(_) => Json(true),
        Err(_) => Json(false), 
    }
}

pub async fn eliminar_pasajero_por_id(State(pool): State<PgPool>, Path(id_pasajero): Path<i32>) -> Json<bool> {
    let pasajeros = pasajero_repository::new(pool);
    match pasajeros.eliminar_pasajero(id_pasajero).await {
        Ok(_) => Json(true),
        Err(_) => Json(false), 
    }
}