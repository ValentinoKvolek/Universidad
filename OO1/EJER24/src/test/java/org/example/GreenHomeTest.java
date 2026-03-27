package org.example;

import org.example.*;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.time.LocalDate;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class GreenHomeTest {

    private Usuario juan;
    private OrdenCompra ordenCompra;
    private OrdenServicio ordenServicio;
    private Producto panelSolar;
    private Producto compostera;
    private Producto calefonSolar;
    private Tecnico lucia;

    @BeforeEach
    public void setUp() {
        // Usuario
        juan = new Usuario("Juan Martínez", "Larrea 5800, Mar del Plata");

        // Productos
        panelSolar = new Producto("Panel solar", "Reciclable", 35000, false);
        compostera = new Producto("Compostera", "Biodegradable", 8000, true);
        calefonSolar = new Producto("Calefón solar", "Reciclable", 50000, false);

        // Técnico
        lucia = new Tecnico("Lucía Iraola", "Instalaciones solares", 4500);

        // Orden de compra
        ordenCompra = new OrdenCompra(LocalDate.now(), juan, juan.getDomicilio(), 0);
        ordenCompra.agregarProducto(panelSolar);
        ordenCompra.agregarProducto(compostera);

        // Orden de servicio
        ordenServicio = new OrdenServicio(LocalDate.now(), juan, juan.getDomicilio(),
                "Instalación de calefón solar", 5);
        ordenServicio.agregarProducto(calefonSolar);
        ordenServicio.agregarTecnico(lucia);

        // Asociar las órdenes al usuario
        juan.agregarOrden(ordenCompra);
        juan.agregarOrden(ordenServicio);
    }

    @Test
    public void testOrdenCompraMenosDe5ProductosSinDescuento() {
        OrdenCompra oc = new OrdenCompra(LocalDate.now(), juan, juan.getDomicilio(), 0);
        for (int i = 0; i < 4; i++) {
            oc.agregarProducto(new Producto("Producto " + i, "Eco", 1000, true));
        }
        assertEquals(4000, oc.calcularCosto(), "Debe ser sin descuento con menos de 5 productos");
    }

    @Test
    public void testOrdenCompraCon5ProductosAplicaDescuento() {
        // valor de borde: exactamente 5 productos
        OrdenCompra oc = new OrdenCompra(LocalDate.now(), juan, juan.getDomicilio(), 0);
        for (int i = 0; i < 5; i++) {
            oc.agregarProducto(new Producto("Producto " + i, "Eco", 1000, true));
        }
        // costo base = 5000 → aplica 10% desc. → 4500
        assertEquals(4500, oc.calcularCosto(), "Debe aplicar 10% de descuento al tener 5 productos");
    }

    @Test
    public void testOrdenServicioCon10HorasSinDescuento() {
        // valor de borde: exactamente 10 horas (sin descuento)
        OrdenServicio os = new OrdenServicio(LocalDate.now(), juan, juan.getDomicilio(),
                "Trabajo estándar", 10);
        os.agregarTecnico(lucia);
        os.agregarProducto(calefonSolar); // 50.000
        // costo = (10 * 4500) + 50000 = 95.000 sin descuento
        assertEquals(95000, os.calcularCosto(), "Debe ser sin descuento con 10 horas");
    }

    @Test
    public void testOrdenServicioMasDe10HorasConDescuento() {
        // partición con descuento (>10 horas)
        OrdenServicio os = new OrdenServicio(LocalDate.now(), juan, juan.getDomicilio(),
                "Trabajo extendido", 11);
        os.agregarTecnico(lucia);
        os.agregarProducto(calefonSolar);
        // costo base = (11 * 4500) + 50000 = 99.500 → con 10% desc. = 89.550
        assertEquals(89550, os.calcularCosto(), "Debe aplicar 10% de descuento con más de 10 horas");
    }
}

