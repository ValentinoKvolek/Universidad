package org.example;

import java.time.LocalDate;

public class PoliticaFlexible implements PoliticaCancelacion {

    @Override
    public double calcularReembolso(Reserva reserva, LocalDate fechaCancelacion, double precioPorNoche) {
        if (fechaCancelacion.isBefore(reserva.getPeriodo().getFrom())) {
            return reserva.calcularPrecio(precioPorNoche);
        }
        return 0;
    }
}
