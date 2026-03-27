package org.example;

import java.time.LocalDate;

public class Retiro implements MetodoEnvio {
    private LocalDate fecha;
    String dni;

    @Override
    public double calcularTiempo() {
        return 0;
    }
}
