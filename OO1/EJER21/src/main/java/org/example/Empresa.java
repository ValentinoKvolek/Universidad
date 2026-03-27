package org.example;

import java.time.LocalDate;

public class Empresa {

    public void agregarEnvio(Cliente cliente, Envio envio){
        cliente.agregarUnEnvio(envio);
    }

    public double montoAPagar(Cliente cliente, LocalDate inicio, LocalDate fin){
        double total =0;
        total += cliente.calcularMontoEnUnPeriodo(inicio,fin);
        return total;
    }


}
