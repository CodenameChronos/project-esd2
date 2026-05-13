use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq,)]
pub struct Pasajero{
    pub id_pasajero: i32,
    pub nombre: String,
    pub pasaporte: String,
    pub nacionalidad: String,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct NuevoPasajero{
    pub nombre: String,
    pub pasaporte: String,
    pub nacionalidad: String,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct ActualizarPasajero{
    pub nombre: String,
    pub pasaporte: String,
    pub nacionalidad: String,
}