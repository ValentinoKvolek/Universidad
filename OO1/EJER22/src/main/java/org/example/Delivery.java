package org.example;

import java.time.LocalDate;

public class Delivery implements MetodoEnvio {
    private LocalDate fecha;
    private String direccion;

    @Override
    public double calcularTiempo() {
        return 0;
    }
}
