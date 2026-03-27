package org.example;

import java.time.LocalDate;


public class Reserva {

    private DateLapse periodo;


    public Reserva(LocalDate desde, LocalDate hasta) {
        this.periodo = new DateLapse(desde, hasta);
    }


    public boolean seSuperponeCon(LocalDate otraDesde, LocalDate otraHasta) {
        DateLapse otroPeriodo = new DateLapse(otraDesde, otraHasta);
        return this.periodo.overlaps(otroPeriodo);
    }

    public double calcularPrecio(double precioPorNoche) {
        return this.periodo.sizeInDays() * precioPorNoche;
    }

    public DateLapse getPeriodo() {
        return this.periodo;
    }


    public boolean yaComenzo() {
        LocalDate hoy = LocalDate.now();
        return this.periodo.includesDate(hoy);
    }


    public boolean estaDentroDePeriodo(DateLapse periodoConsulta) {
        LocalDate desdeConsulta = periodoConsulta.getFrom();
        LocalDate hastaConsulta = periodoConsulta.getTo();

        return !(this.periodo.getTo().isBefore(desdeConsulta) || this.periodo.getFrom().isAfter(hastaConsulta));
    }

}
