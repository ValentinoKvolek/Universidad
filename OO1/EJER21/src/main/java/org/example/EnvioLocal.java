package org.example;

import java.time.LocalDate;

public class EnvioLocal extends Envio{

    public EnvioLocal(LocalDate fecha, String origen, String destino, double peso, boolean entregaRapida) {
        super(fecha, origen, destino, peso, entregaRapida);
    }

    @Override
    public double costoEnvio() {
        if(entregaRapida){
            return 1500;
        }
        return 1000;
    }
}
