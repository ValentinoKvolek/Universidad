package org.example;

import java.time.LocalDate;

public class PoliticaModerada implements PoliticaCancelacion {

    @Override
    public double calcularReembolso(Reserva reserva, LocalDate fechaCancelacion, double precioPorNoche) {
        LocalDate inicio = reserva.getPeriodo().getFrom();
        if (fechaCancelacion.isBefore(inicio.minusDays(7))) {
            return reserva.calcularPrecio(precioPorNoche); // 100%
        } else if (fechaCancelacion.isBefore(inicio.minusDays(2))) {
            return reserva.calcularPrecio(precioPorNoche) * 0.5; // 50%
        }
        return 0;
    }
}
