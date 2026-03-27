package org.example;

import java.time.LocalDate;
import java.util.ArrayList;
import java.util.List;

public class OrdenServicio extends Orden {
    private String descripcion;
    private int horas;
    private List<Tecnico> tecnicos;

    public OrdenServicio(LocalDate fecha, Usuario usuario, String domicilio, String descripcion, int horas) {
        super(fecha, usuario, domicilio);
        this.descripcion = descripcion;
        this.horas = horas;
        this.tecnicos = new ArrayList<>();
    }

    public void agregarTecnico(Tecnico tecnico) {
        this.tecnicos.add(tecnico);
    }

    @Override
    public double calcularCosto() {
        double totalProductos = 0;
        for (Producto p : productos) {
            totalProductos += p.getCosto();
        }

        double totalTrabajo = 0;
        for (Tecnico t : tecnicos) {
            totalTrabajo += t.getValorHora() * horas;
        }

        return totalProductos + totalTrabajo;
    }
}

