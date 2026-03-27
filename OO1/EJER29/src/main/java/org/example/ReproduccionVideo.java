package org.example;

import java.time.LocalDate;

public class ReproduccionVideo extends Actividad {
    Integer duracionTotal;
    Integer duracionPublicidad;

    public ReproduccionVideo(String ip, LocalDate fechaInicio, Integer duracionTotal, Integer duracionPublicidad) {
        super(ip, fechaInicio);
        this.duracionTotal = duracionTotal;
        this.duracionPublicidad = duracionPublicidad;
    }

    @Override
    public double montoIntervalo(LocalDate fechaI, LocalDate fechaF) {
        if (this.fechaInicio.isBefore(fechaI) || this.fechaInicio.isAfter(fechaF)){
            return 0.0; //si esta fuera del intervalo no devuelvo nada
        }
        int duracionReal = duracionTotal - duracionPublicidad;
        return  10 * duracionReal;
    }

    @Override
    public boolean estaEnIntervalo(LocalDate fechaI, LocalDate fechaF) {
        if (this.fechaInicio.isBefore(fechaI) || this.fechaInicio.isAfter(fechaF)){
            return false;
        }
        return true;
    }

}
