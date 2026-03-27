package org.example;

import org.example.Orden;

import java.time.LocalDate;

public class OrdenCompra extends Orden {
    private double precioEnvio;

    public OrdenCompra(LocalDate fecha, Usuario usuario, String domicilio, double precioEnvio) {
        super(fecha, usuario, domicilio);
        this.precioEnvio = precioEnvio;
    }

    @Override
    public double calcularCosto() {
        double totalProductos = 0;
        for (Producto p : productos) {
            totalProductos += p.getCosto();
        }
        return totalProductos + precioEnvio;
    }
}