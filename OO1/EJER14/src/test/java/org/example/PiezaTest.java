package org.example;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

public class PiezaTest {

    private ReporteDeConstruccion reporte;

    @BeforeEach
    void setUp() {
        reporte = new ReporteDeConstruccion();

        Cilindro c = new Cilindro();
        c.radio = 2;
        c.altura = 5;
        c.material = "Hierro";
        c.color = "Rojo";

        // Esfera de hierro azul
        Esferas e = new Esferas();
        e.radio = 3;
        e.material = "Hierro";
        e.color = "Azul";

        PrismasRectangulares p = new PrismasRectangulares();
        p.ladoMayor = 2;
        p.ladoMenor = 3;
        p.altura = 4;
        p.material = "Aluminio";
        p.color = "Rojo";

        reporte.agregarPieza(c);
        reporte.agregarPieza(e);
        reporte.agregarPieza(p);
    }

    @Test
    void testVolumenDeMaterial() {
        double esperadoHierro =
                (Math.PI * Math.pow(2, 2) * 5) + ((4.0 / 3.0) * Math.PI * Math.pow(3, 3));

        assertEquals(esperadoHierro, reporte.volumenDeMaterial("Hierro"), 0.0001);
    }

    @Test
    void testSuperficieDeColor() {
        double superficieCilindro = 2 * Math.PI * 2 * 5 + 2 * Math.PI * Math.pow(2, 2);
        double superficiePrisma = 2 * (2 * 3 + 2 * 4 + 3 * 4);
        double esperadoRojo = superficieCilindro + superficiePrisma;

        assertEquals(esperadoRojo, reporte.superficieDeColor("Rojo"), 0.0001);
    }

    @Test
    void testVolumenConMaterialInexistente() {
        assertEquals(0, reporte.volumenDeMaterial("Madera"));
    }

    @Test
    void testSuperficieConColorInexistente() {
        assertEquals(0, reporte.superficieDeColor("Verde"));
    }
}
