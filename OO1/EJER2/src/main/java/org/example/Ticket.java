package org.example;

import org.example.Producto;

import java.time.LocalDate;
import java.util.ArrayList;
import java.util.List;

public class Ticket {

    private LocalDate fecha;
    private double pesoTotal;
    private int cantidadDeProductos;
    private double precioTotal;
    private List<Producto> productos;


    public Ticket(LocalDate fecha, double pesoTotal, int cantidadDeProductos, double precioTotal, List<Producto> productos) {
        this.fecha = fecha;
        this.pesoTotal = pesoTotal;
        this.cantidadDeProductos = cantidadDeProductos;
        this.precioTotal = precioTotal;
        this.productos = productos;
    }

    public LocalDate getFecha() {
        return fecha;
    }

    public double getPesoTotal() {
        return pesoTotal;
    }

    public int getCantidadDeProductos() {
        return cantidadDeProductos;
    }

    public double getPrecioTotal() {
        return precioTotal;
    }

    public Ticket() {
        this.productos = new ArrayList<>();
    }

    public void setFecha(LocalDate fecha) {
        this.fecha = fecha;
    }

    public void setPesoTotal(double pesoTotal) {
        this.pesoTotal = pesoTotal;
    }

    public void setCantidadDeProductos(int cantidadDeProductos) {
        this.cantidadDeProductos = cantidadDeProductos;
    }

    public void setPrecioTotal(double precioTotal) {
        this.precioTotal = precioTotal;
    }

    // Nuevo: guardar la lista de productos
    public void setProductos(List<Producto> productos) {
        this.productos = new ArrayList<>(productos); // copia defensiva
    }

    public List<Producto> getProductos() {
        return new ArrayList<>(productos); // devolución segura
    }

    // El impuesto se sigue calculando igual que antes
    public double impuesto() {
        return this.precioTotal * 0.21;
    }
    // getters para fecha, peso, etc. si los necesitás
}
