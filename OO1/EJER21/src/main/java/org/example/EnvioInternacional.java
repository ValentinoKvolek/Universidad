package org.example;

import java.time.LocalDate;

public class EnvioInternacional extends  Envio{

    public EnvioInternacional(LocalDate fecha, String origen, String destino, double peso, boolean entregaRapida) {
        super(fecha, origen, destino, peso, entregaRapida);
    }

    @Override
    public double costoEnvio() {
        int total =5000;
        if(entregaRapida){
            total+=800;
        }
        if(this.peso <= 1000){
            return total + (10 * this.peso);
        }
        return total + (12 * this.peso);
    }
}
