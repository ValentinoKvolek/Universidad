package org.example;

import java.time.Instant;
import java.time.LocalDate;

public abstract class Actividad {

    LocalDate fechaInicio;
    String ip;

    public Actividad(String ip, LocalDate fechaInicio) {
        this.ip = ip;
        this.fechaInicio = fechaInicio;
    }

    public abstract double montoIntervalo(LocalDate fechaI, LocalDate fechaF);

    public LocalDate getFechaInicio() {
        return this.fechaInicio;
    }

    public String getIp() {
        return  this.ip;
    }
    public abstract boolean estaEnIntervalo(LocalDate fechaI, LocalDate fechaF);
}
