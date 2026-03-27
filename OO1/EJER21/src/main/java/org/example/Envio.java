package org.example;

import java.time.LocalDate;

public abstract class Envio {
    protected LocalDate fecha;
    protected String origen;
    protected String destino;
    protected double peso;
    protected boolean entregaRapida;

    public Envio(LocalDate fecha, String origen, String destino, double peso, boolean entregaRapida) {
        this.fecha = fecha;
        this.origen = origen;
        this.destino = destino;
        this.peso = peso;
        this.entregaRapida = entregaRapida;
    }

    public boolean estaEnElPeriodo(LocalDate inicio, LocalDate fin){
        return (fecha.isEqual(inicio) || fecha.isAfter(inicio)) &&
                (fecha.isEqual(fin)    || fecha.isBefore(fin));
    }

    public abstract double costoEnvio();
}
