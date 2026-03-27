package org.example;

import java.time.LocalDate;

public interface Plan {
    public double calcularPrecioBase();
    public double montoPenalizacion(int cantIps, int antiguedad);

}



