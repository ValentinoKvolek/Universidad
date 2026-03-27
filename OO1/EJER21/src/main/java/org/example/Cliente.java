package org.example;

import java.time.LocalDate;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;

public abstract class Cliente {

    protected List<Envio> envios = new ArrayList<>();

    public void agregarUnEnvio(Envio envio){
        this.envios.add(envio);
    }

    protected abstract double aplicarDescuento(double total);

    public double calcularMontoEnUnPeriodo(LocalDate inicio, LocalDate fin){
        double total=0;

        for (Envio e : envios){
            if(e.estaEnElPeriodo(inicio,fin)){
                total+= e.costoEnvio();
            }
        }

        return aplicarDescuento(total);
    }
}
