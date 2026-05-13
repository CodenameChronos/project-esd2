CREATE TABLE Aeropuertos (
    id_aeropuerto SERIAL PRIMARY KEY,
    codigo_iata VARCHAR(3) UNIQUE NOT NULL,
    nombre VARCHAR(100),
    ciudad VARCHAR(100)
);

CREATE TABLE Aviones (
    id_avion SERIAL PRIMARY KEY,
    modelo VARCHAR(50) NOT NULL,
    capacidad_pasajeros INT NOT NULL,
    fabricante VARCHAR(50)
);

CREATE TABLE Vuelos (
    id_vuelo SERIAL PRIMARY KEY,
    numero_vuelo VARCHAR(10) UNIQUE NOT NULL,
    id_aeropuerto_origen INT REFERENCES Aeropuertos(id_aeropuerto),
    id_aeropuerto_destino INT REFERENCES Aeropuertos(id_aeropuerto),
    id_avion INT REFERENCES Aviones(id_avion)
);

CREATE TABLE Pasajeros (
    id_pasajero SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    pasaporte VARCHAR(20) UNIQUE NOT NULL,
    nacionalidad VARCHAR(50)
);

CREATE TABLE Reservas (
    id_reserva SERIAL PRIMARY KEY,
    id_vuelo INT REFERENCES Vuelos(id_vuelo),
    id_pasajero INT REFERENCES Pasajeros(id_pasajero),
    asiento VARCHAR(5),
    precio_boleto DECIMAL(10,2)
);