use sqlx::{PgPool, Row};
use crate::models::vuelo::{Vuelo, NuevoVuelo, ActualizarVuelo};

pub struct VuelosRepository {
    pool: PgPool,
}

impl VuelosRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn obtener_vuelos(&self) -> sqlx::Result<Vec<Vuelo>> {
        let filas = sqlx::query("SELECT id_vuelo, numero_vuelo, id_aeropuerto_origen, id_aeropuerto_destino, id_avion FROM Vuelos")
            .fetch_all(&self.pool)
            .await?;
        let vuelos = filas.into_iter().map (|fila|{
            Vuelo {
                id_vuelo: fila.get("id_vuelo"),
                numero_vuelo: fila.get("numero_vuelo"),
                id_aeropuerto_origen: fila.get("id_aeropuerto_origen"),
                id_aeropuerto_destino: fila.get("id_aeropuerto_destino"),
                id_avion: fila.get("id_avion"),
            }
        }).collect();
        Ok(vuelos)
    }

    pub async fn crear_vuelo(&self, nuevo_vuelo: NuevoVuelo) -> sqlx::Result<Vuelo> {
        let fila = sqlx::query("INSERT INTO Vuelos (numero_vuelo, id_aeropuerto_origen, id_aeropuerto_destino, id_avion) VALUES ($1, $2, $3, $4) RETURNING id_vuelo, numero_vuelo, id_aeropuerto_origen, id_aeropuerto_destino, id_avion")
            .bind(nuevo_vuelo.numero_vuelo)
            .bind(nuevo_vuelo.id_aeropuerto_origen)
            .bind(nuevo_vuelo.id_aeropuerto_destino)
            .bind(nuevo_vuelo.id_avion)
            .fetch_one(&self.pool)
            .await?;
        let id_vuelo: i32 = fila.get("id_vuelo");
        Ok(Vuelo {
            id_vuelo,
            numero_vuelo: nuevo_vuelo.numero_vuelo,
            id_aeropuerto_origen: nuevo_vuelo.id_aeropuerto_origen,
            id_aeropuerto_destino: nuevo_vuelo.id_aeropuerto_destino,
            id_avion: nuevo_vuelo.id_avion,
        })
    }

    pub async fn actualizar_vuelo(&self, id_vuelo: i32, vuelo_actualizado: ActualizarVuelo) -> sqlx::Result<Vuelo> {
        let fila = sqlx::query("UPDATE Vuelos SET numero_vuelo = $1, id_aeropuerto_origen = $2, id_aeropuerto_destino = $3, id_avion = $4 WHERE id_vuelo = $5 RETURNING id_vuelo, numero_vuelo, id_aeropuerto_origen, id_aeropuerto_destino, id_avion")
            .bind(vuelo_actualizado.numero_vuelo)
            .bind(vuelo_actualizado.id_aeropuerto_origen)
            .bind(vuelo_actualizado.id_aeropuerto_destino)
            .bind(vuelo_actualizado.id_avion)
            .bind(id_vuelo)
            .fetch_one(&self.pool)
            .await?;
        Ok(Vuelo {
            id_vuelo: fila.get("id_vuelo"),
            numero_vuelo: fila.get("numero_vuelo"),
            id_aeropuerto_origen: fila.get("id_aeropuerto_origen"),
            id_aeropuerto_destino: fila.get("id_aeropuerto_destino"),
            id_avion: fila.get("id_avion"),
        })
    }

    pub async fn eliminar_vuelo(&self, id_vuelo: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Vuelos WHERE id_vuelo = $1")
            .bind(id_vuelo)
            .execute(&self.pool)
            .await?;
        Ok(())
        }
}