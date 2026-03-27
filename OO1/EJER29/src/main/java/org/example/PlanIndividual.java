package org.example;

import java.time.LocalDate;

public class PlanIndividual implements Plan {
    public PlanIndividual(Integer cantMinutosContratados) {
        this.cantMinutosContratados = cantMinutosContratados;
    }

    Integer cantMinutosContratados;

    @Override
    public double calcularPrecioBase() {
        return 20 * cantMinutosContratados;
    }

    @Override
    public double montoPenalizacion(int cantIps, int antiguedad) {
        if(cantIps > 1 ){
            return (cantIps - 1) * 300;
        }
        return 0.0;
    }
}



