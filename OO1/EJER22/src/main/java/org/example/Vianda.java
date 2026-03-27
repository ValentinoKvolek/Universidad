package org.example;

public class Vianda implements Producto {
    String nombre;
    double precio;
    boolean aptaCeliaco;
    boolean disponibilidad;

    @Override
    public double costo() {
        return 0;
    }
}
