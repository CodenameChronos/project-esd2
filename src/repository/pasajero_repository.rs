use sqlx::{PgPool, Row};
use crate::models::pasajero::{Pasajero, NuevoPasajero, ActualizarPasajero};

pub struct PasajerosRepository {
    pool: PgPool,
}

impl PasajerosRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_pasajeros(&self) -> sqlx::Result<Vec<Pasajero>> {
        let filas = sqlx::query("SELECT id_pasajero, nombre, pasaporte, nacionalidad FROM Pasajeros")
            .fetch_all(&self.pool)
            .await?;
        let pasajeros = filas.into_iter().map (|fila|{
            Pasajero {
                id_pasajero: fila.get("id_pasajero"),
                nombre: fila.get("nombre"),
                pasaporte: fila.get("pasaporte"),
                nacionalidad: fila.get("nacionalidad"),
            }
        }).collect();
        Ok(pasajeros)
    }

    pub async fn crear_pasajero(&self, nuevo_pasajero: NuevoPasajero) -> sqlx::Result<Pasajero> {
        let fila = sqlx::query("INSERT INTO Pasajeros (nombre, pasaporte, nacionalidad) VALUES ($1, $2, $3) RETURNING id_pasajero, nombre, pasaporte, nacionalidad")
            .bind(nuevo_pasajero.nombre)
            .bind(nuevo_pasajero.pasaporte)
            .bind(nuevo_pasajero.nacionalidad)
            .fetch_one(&self.pool)
            .await?;
        let id_pasajero: i32 = fila.get("id_pasajero");
        Ok(Pasajero {
            id_pasajero,
            nombre: nuevo_pasajero.nombre,
            pasaporte: nuevo_pasajero.pasaporte,
            nacionalidad: nuevo_pasajero.nacionalidad,
        })
    }

    pub async fn actualizar_pasajero(&self, id_pasajero: i32, pasajero_actualizado: ActualizarPasajero) -> sqlx::Result<Pasajero> {
        let fila = sqlx::query("UPDATE Pasajeros SET nombre = $1, pasaporte = $2, nacionalidad = $3 WHERE id_pasajero = $4 RETURNING id_pasajero, nombre, pasaporte, nacionalidad")
            .bind(pasajero_actualizado.nombre)
            .bind(pasajero_actualizado.pasaporte)
            .bind(pasajero_actualizado.nacionalidad)
            .bind(id_pasajero)
            .fetch_one(&self.pool)
            .await?;
        Ok(Pasajero {
            id_pasajero: fila.get("id_pasajero"),
            nombre: fila.get("nombre"),
            pasaporte: fila.get("pasaporte"),
            nacionalidad: fila.get("nacionalidad"),
        })
    }

    pub async fn eliminar_pasajero(&self, id_pasajero: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Pasajeros WHERE id_pasajero = $1")
            .bind(id_pasajero)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

}