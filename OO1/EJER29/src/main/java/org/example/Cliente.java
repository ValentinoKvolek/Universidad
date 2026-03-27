package org.example;

import java.time.LocalDate;
import java.time.Period;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;

public class Cliente {

    String nombre;
    LocalDate fechaAlta;
    Plan plan;
    List<Actividad> actividades;

    public Cliente(String nombre, LocalDate fechaAlta, Plan plan, List<Actividad> actividades) {
        this.nombre = nombre;
        this.fechaAlta = fechaAlta;
        this.plan = plan;
        this.actividades = actividades;
    }

    public double montoTotal(LocalDate fechaInicio, LocalDate fechaFin){
        double total = 0.0;
        total+= plan.calcularPrecioBase();
        for(Actividad a : actividades){
           total += a.montoIntervalo(fechaInicio,fechaFin);
        }
        total+= montoPenalizacion(fechaInicio,fechaFin);
        return total;
    }

    public double montoPenalizacion(LocalDate fechaInicio, LocalDate fechaFin){

        int cantIps = contarIpsIntervalo(fechaInicio,fechaFin);
        int antiguedad = Period.between(this.fechaAlta, fechaFin).getYears();

        return plan.montoPenalizacion(cantIps,antiguedad);

    }

    public int contarIpsIntervalo(LocalDate fechaInicio, LocalDate fechaFin) {
        Set<String> ips = new HashSet<>();

        for (Actividad a : actividades) {
            if (a.estaEnIntervalo(fechaInicio, fechaFin)) {
                ips.add(a.getIp());
            }
        }

        return ips.size();
    }

}
