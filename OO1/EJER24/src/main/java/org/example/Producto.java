package org.example;

public class Producto {
    private String nombre;
    private String tipoMaterial;
    private double costo;
    private boolean esBiodegradable;

    public Producto(String nombre, String tipoMaterial, double costo, boolean esBiodegradable) {
        this.nombre = nombre;
        this.tipoMaterial = tipoMaterial;
        this.costo = costo;
        this.esBiodegradable = esBiodegradable;
    }

    public String getNombre() {
        return nombre;
    }

    public double getCosto() {
        return costo;
    }


}
