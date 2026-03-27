package org.example;

import java.time.LocalDate;

public interface PoliticaCancelacion {
    double calcularReembolso(Reserva reserva, LocalDate fechaCancelacion, double precioPorNoche);
}
