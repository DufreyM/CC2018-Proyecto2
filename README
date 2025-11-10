# 🌌 Proyecto 3D — Escena interactiva en Rust

---

## Ejecución

### Requisitos

### Comando
```bash
cargo run --release
```

---

## 🎮 Controles e interacción

| Tecla | Acción |
|-------|--------|
| `W` | Acercar cámara (zoom in) |
| `S` | Alejar cámara (zoom out) |
| `←` / `→` | Rotar cámara horizontalmente |
| `↑` / `↓` | Rotar cámara verticalmente |
| `P` | Mover el ciclo hacia la **noche** |
| `O` | Mover el ciclo hacia el **día** |
| `ESC` | Salir del programa |

---

## ☀️ Ciclo Día/Noche

- El sistema `day_night_cycle` controla **la posición del sol**, **la luz ambiental** y **la intensidad de la iluminación**.
- El sol orbita lentamente sobre la escena.
- Puedes avanzar o retroceder manualmente con `Q` y `E`.

Durante el día:
- El ambiente es cálido y brillante.  
Durante la noche:
- El color ambiental se vuelve azulado y suave, con luz tenue.

---

## 🏠 Elementos de la escena

### 🏡 Casa
- Construida con bloques de pastel y nidos de abeja
- Tiene una **ventana de cristal transparente** y **techo escalonado**.

### 🌳 Árbol
- Es un champiñon
- Añade profundidad natural al entorno.

### 🌸 Flores (Azale)
- Textura: `azaleas.png`.
- Pequeños cubos translúcidos con color rosado brillante.

### 🌀 Portal mágico
- Material translúcido (`PORTAL`).
- Leve transparencia y brillo, animado con movimiento de textura.
- Simula energía flotante frente a la casa.

---

## ⚙️ Rendimiento y threads

- El render se ejecuta con **threads paralelos** para mejorar FPS.
- En promedio, la escena corre entre **30 y 60 FPS** dependiendo del hardware.

---

## 🧠 Créditos

Proyecto desarrollado por **Leonardo Mejía**  
Curso: *Computación Gráfica — CC2028*  
Universidad del Valle de Guatemala  

---

> 💡 *Tip:* si tu FPS baja, ejecuta `cargo run --release` o reduce la resolución del framebuffer.