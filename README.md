# MeshNode - Registro de nodos DePIN en Solana

MeshNode es un programa backend desarrollado en **Solana** utilizando **Rust** y **Anchor**.  
El proyecto permite registrar una red DePIN y nodos asociados a una wallet mediante el uso de **PDA (Program Derived Addresses)**.

Este proyecto fue desarrollado como una prueba de concepto enfocada en demostrar el uso de:

- CRUD en Solana
- Cuentas on-chain
- PDA
- Validaciones básicas de acceso
- Desarrollo backend con Rust y Anchor

---

## Descripción del proyecto

El programa permite administrar una red descentralizada de nodos de conectividad comunitaria.

Cada usuario puede:

- Crear una red
- Consultar la red creada
- Actualizar los datos de la red
- Eliminar la red si no tiene nodos asociados
- Registrar un nodo dentro de la red
- Consultar la información del nodo
- Actualizar la información del nodo
- Alternar el estado del nodo
- Eliminar el nodo

La lógica del proyecto está orientada a una estructura simple tipo DePIN, donde existe una red principal y nodos asociados a wallets.

---

## Objetivo

El objetivo de MeshNode es demostrar cómo se puede construir un backend en Solana para registrar y gestionar infraestructura física descentralizada mediante cuentas PDA.

---

## Tecnologías utilizadas

- **Solana**
- **Rust**
- **Anchor**
- **Solana Playground**
- **Phantom / Playground Wallet**
- **Devnet**

---

## Estructura del proyecto

El proyecto utiliza dos cuentas principales:

### 1. Red
Cuenta principal que almacena la información de la red.

Campos:
- `authority: Pubkey`
- `nombre: String`
- `descripcion: String`
- `total_nodos: u32`
- `activa: bool`

### 2. Nodo
Cuenta que almacena la información de un nodo asociado a una red y a una wallet.

Campos:
- `red: Pubkey`
- `owner: Pubkey`
- `alias: String`
- `tipo: String`
- `ubicacion: String`
- `activo: bool`
- `fecha_registro: i64`

---

## CRUD implementado

### Red
- `crear_red`
- `ver_red`
- `actualizar_red`
- `eliminar_red`

### Nodo
- `registrar_nodo`
- `ver_nodo`
- `actualizar_nodo`
- `alternar_estado_nodo`
- `eliminar_nodo`

---

## PDA utilizadas

### PDA de Red
La cuenta de la red se deriva con las siguientes seeds:

- `"red"`
- `authority`

Representación:

```rust
seeds = [b"red", authority.key().as_ref()]
