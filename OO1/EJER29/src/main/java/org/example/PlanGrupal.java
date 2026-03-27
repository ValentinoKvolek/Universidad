package org.example;

import java.time.LocalDate;
import java.time.Period;

public class PlanGrupal implements Plan {

    Integer cantMaxIps;

    public PlanGrupal(Integer cantMaxIps) {
        this.cantMaxIps = cantMaxIps;
    }

    @Override
    public double calcularPrecioBase() {
        return 800 * cantMaxIps;
    }

    @Override
    public double montoPenalizacion(int cantIps, int antiguedad) {
        if (cantIps > this.cantMaxIps) {
            if (antiguedad > 10) {
                return 0.0;
            } else {
                return (cantIps - this.cantMaxIps) * 500;
            }
        }
        return 0.0;
    }

}
