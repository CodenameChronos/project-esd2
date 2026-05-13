use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq,)]
pub struct Vuelo{
    pub id_vuelo: i32,
    pub numero_vuelo: String,
    pub id_aeropuerto_origen: i32,
    pub id_aeropuerto_destino: i32,
    pub id_avion: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct NuevoVuelo{
    pub numero_vuelo: String,
    pub id_aeropuerto_origen: i32,
    pub id_aeropuerto_destino: i32,
    pub id_avion: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct ActualizarVuelo{
    pub numero_vuelo: String,
    pub id_aeropuerto_origen: i32,
    pub id_aeropuerto_destino: i32,
    pub id_avion: i32,
}