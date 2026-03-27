package org.example;

import java.time.LocalDate;
import java.util.List;

public class SesionDeJuego  extends  Actividad {

    Integer duracion;
    List<Item> items;

    public SesionDeJuego(String ip, LocalDate fechaInicio, Integer duracion, List<Item> items) {
        super(ip, fechaInicio);
        this.duracion = duracion;
        this.items = items;
    }

    @Override
    public double montoIntervalo(LocalDate fechaI, LocalDate fechaF) {
        if (this.fechaInicio.isBefore(fechaI) || this.fechaInicio.isAfter(fechaF)){
            return 0.0; //si esta fuera del intervalo no devuelvo nada
        }
        if(duracion > 360 ){
            Double total = 0.0;
            for(Item i:items){
                total+= i.precioFinal();
            }
            return total;
        }
        return 0.0;
    }

    @Override
    public boolean estaEnIntervalo(LocalDate fechaI, LocalDate fechaF) {
        if (this.fechaInicio.isBefore(fechaI) || this.fechaInicio.isAfter(fechaF)){
            return false;
        }
        return true;
    }
}
