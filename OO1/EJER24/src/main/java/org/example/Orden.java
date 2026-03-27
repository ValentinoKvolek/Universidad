package org.example;

import java.time.LocalDate;
import java.util.ArrayList;
import java.util.List;

abstract class Orden {
    protected LocalDate fecha;
    protected Usuario usuario;
    protected String domicilio;
    protected List<Producto> productos;

    public Orden(LocalDate fecha, Usuario usuario, String domicilio) {
        this.fecha = fecha;
        this.usuario = usuario;
        this.domicilio = domicilio;
        this.productos = new ArrayList<>();
    }

    public void agregarProducto(Producto producto) {
        this.productos.add(producto);
    }

    public abstract double calcularCosto();
}

