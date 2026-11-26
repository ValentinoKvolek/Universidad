# 🚀 CADP Final Master - Plataforma Interactiva de Estudio (UNLP)

> **Super-resumen interactivo y animado para preparar y aprobar el Examen Final de Conceptos de Algoritmos, Datos y Programación (Facultad de Informática - UNLP).**

Desarrollado por [Valentino Kvolek](https://github.com/ValentinoKvolek).

---

## 📌 ¿Por qué esta plataforma?
El final de CADP es cualitativamente muy distinto a los parciales:
1. **El Ejercicio 1 (Práctica) es eliminatorio:** Si el algoritmo de punteros, listas o corte de control tiene un error conceptual, la cátedra no corrige la teoría.
2. **La Teoría es estricta:** Trazas con aliasing y shadowing, cálculo exacto de bytes (Stack vs Heap), fórmulas de UDT en bucles y justificaciones rigurosas de Verdadero/Falso con palabras clave obligatorias.

---

## 🌟 Módulos Incluidos

### 1. ⚡ Laboratorio de Algoritmos ("El Portero del Final")
Visualización animada paso a paso con código Pascal sincronizado:
- **Inserción ordenada en lista:** punteros `ant`, `act`, `nue`, avance coordinado y enlaces finales.
- **Eliminación de duplicados:** uso obligatorio de `aux` y `dispose(aux)` (ej. Final Mayo 2026).
- **Corte de control:** patrón clásico de final para agrupar elementos ordenados en una sola pasada $O(N)$.
- **Búsqueda dicotómica en vector:** cálculo de `medio` y ajuste de intervalos `pri`/`ult`.
- **Inserción con corrimiento en vector:** desplazamiento hacia la derecha con bucle `downto`.

### 2. 🧠 Simulador de Memoria (Stack vs Heap)
- **Casos reales de final:** Fuga de memoria con `new(p)` seguido de `p := nil`, detona de **Run-time Error 204** por `dispose(nil)`, y punteros colgantes (*dangling pointers*).
- **Sandbox interactivo:** Accioná libremente `new`, `dispose`, `nil` y asignaciones de punteros viendo su impacto en vivo en Pila y Montículo.

### 3. 🔍 Trazador de Ejecución con Aliasing & Shadowing
- Traza del **Final Agosto 2025 Tema 2** (`numero(b, c)` con aliasing entre parámetro `var` y variable global).
- Traza del **Final Marzo 2026** (pasaje por valor vs referencia y ocultamiento de globales).

### 4. 📐 Calculadora de UDT y Desglose de Memoria en Bytes
- Fórmulas oficiales de la cátedra para asignaciones, condiciones, bucles `FOR`, `WHILE` y `REPEAT..UNTIL`.
- Desglose de memoria estática ($342\text{ bytes}$) y dinámica ($2.926\text{ bytes}$) del examen de Febrero 2024.

### 5. 🎯 Entrenador de Verdadero / Falso (Modo Cátedra)
- Más de 50 afirmaciones reales con justificaciones académicas y resaltado de **palabras clave obligatorias**.

### 6. 📖 Resumen Canónico
- Síntesis de los 8 módulos del programa de la UNLP con buscador en tiempo real.

### 7. 🏆 Banco de Finales Resueltos (2022 - 2026)
- Exámenes completos con enunciados, resoluciones en Pascal y criterios de corrección docente.

---

## 🛠️ Tecnologías Utilizadas
- **React 19**
- **Vite 6**
- **Tailwind CSS**
- **Lucide React** (iconografía)
- **KaTeX** (renderizado de fórmulas matemáticas)

---

## 💻 Cómo Ejecutar Localmente

```bash
# 1. Ingresar a la carpeta
cd "resumen cadp"

# 2. Instalar dependencias
npm install

# 3. Iniciar el servidor de desarrollo
npm run dev

# O compilar y previsualizar para producción:
npm run build
npm run preview
```

---
*Hecho para estudiantes de la Facultad de Informática - Universidad Nacional de La Plata (UNLP).*
