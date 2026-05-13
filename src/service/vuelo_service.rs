use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::vuelo::{Vuelo, NuevoVuelo, ActualizarVuelo};
use crate::repository::vuelo_repository;

pub async fn obtener_vuelos(State(pool): State<PgPool>) -> Json<Vec<crate::models::vuelo::Vuelo>> {
    let vuelos = vuelo_repository::new(pool);
    match vuelos.obtener_vuelos().await {
        Ok(vuelos) => Json(vuelos),
        Err(_) => Json(vec![]), 
    }
}

pub async fn crear_vuelo(State(pool): State<PgPool>, Json(nuevo_vuelo): Json<NuevoVuelo>) -> Json<Option<Vuelo>> {
    let vuelos = vuelo_repository::new(pool);
    match vuelos.crear_vuelo(nuevo_vuelo).await {
        Ok(vuelo) => Json(Some(vuelo)),
        Err(_) => Json(Vuelo {
            id_vuelo: 0,
            numero_vuelo: "Error al crear vuelo".to_string(),
            id_aeropuerto_origen: 0,
            id_aeropuerto_destino: 0,
            id_avion: 0,
        }), 
    }
}

pub async fn actualizar_vuelo(State(pool): State<PgPool>, Json(vuelo_actualizado): Json<crate::models::vuelo::Vuelo>) -> Json<crate::models::vuelo::Vuelo> {
    let vuelos = vuelo_repository::new(pool);
    let id = vuelo_actualizado.id_vuelo;
    let nuevo_vuelo = NuevoVuelo {
        numero_vuelo: vuelo_actualizado.numero_vuelo,
        id_aeropuerto_origen: vuelo_actualizado.id_aeropuerto_origen,
        id_aeropuerto_destino: vuelo_actualizado.id_aeropuerto_destino,
        id_avion: vuelo_actualizado.id_avion,
    };
    match vuelos.actualizar_vuelo(id, nuevo_vuelo).await {
        Ok(vuelo) => Json(vuelo),
        Err(_) => Json(Vuelo {
            id_vuelo: 0,
            numero_vuelo: "Error al actualizar vuelo".to_string(),
            id_aeropuerto_origen: 0,
            id_aeropuerto_destino: 0,
            id_avion: 0,
        }), 
    }
}

pub async fn actualizar_vuelo_por_id(State(pool): State<PgPool>, Path(id_vuelo): Path<i32>, Json(vuelo_actualizado): Json<ActualizarVuelo>) -> Json<crate::models::vuelo::Vuelo> {
    let vuelos = vuelo_repository::new(pool);
    let nuevo_vuelo = NuevoVuelo {
        numero_vuelo: vuelo_actualizado.numero_vuelo,
        id_aeropuerto_origen: vuelo_actualizado.id_aeropuerto_origen,
        id_aeropuerto_destino: vuelo_actualizado.id_aeropuerto_destino,
        id_avion: vuelo_actualizado.id_avion,
    };
    match vuelos.actualizar_vuelo(id_vuelo, nuevo_vuelo).await {
        Ok(vuelo) => Json(vuelo),
        Err(_) => Json(Vuelo {
            id_vuelo: 0,
            numero_vuelo: "Error al actualizar vuelo".to_string(),
            id_aeropuerto_origen: 0,
            id_aeropuerto_destino: 0,
            id_avion: 0,
        }), 
    }
}

pub async fn eliminar_vuelo(State(pool): State<PgPool>, Path(id_vuelo): Path<i32>) -> Json<bool> {
    let vuelos = vuelo_repository::new(pool);
    match vuelos.eliminar_vuelo(id_vuelo).await {
        Ok(_) => Json(true),
        Err(_) => Json(false), 
    }
}

pub async fn eliminar_vuelo_por_id(State(pool): State<PgPool>, Path(id_vuelo): Path<i32>) -> Json<bool> {
    let vuelos = vuelo_repository::new(pool);
    match vuelos.eliminar_vuelo_por_id(id_vuelo).await {
        Ok(_) => Json(true),
        Err(_) => Json(false), 
    }
}