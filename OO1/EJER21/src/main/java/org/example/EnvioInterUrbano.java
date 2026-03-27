package org.example;

import java.time.LocalDate;

public class EnvioInterUrbano  extends  Envio{

    Integer distancia;

    public EnvioInterUrbano(LocalDate fecha, String origen, String destino, double peso, boolean entregaRapida, Integer distancia) {

        super(fecha, origen, destino, peso, entregaRapida);
        this.distancia=distancia;
    }

    @Override
    public double costoEnvio() {

        if (this.distancia < 100){
           return 20*this.peso;
        }else if(this.distancia >100 && this.distancia < 500){
            return 25*this.peso;
        }

        return 30*this.peso;
    }
}
