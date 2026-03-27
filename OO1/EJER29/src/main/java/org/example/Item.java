package org.example;

public class Item {
    String nombre;
    Integer cantidad;
    Double precioUnitario;

    public Item(String nombre, Integer cantidad, Double precioUnitario) {
        this.nombre = nombre;
        this.cantidad = cantidad;
        this.precioUnitario = precioUnitario;
    }

    public double precioFinal(){
        return cantidad * precioUnitario;
    }
}
