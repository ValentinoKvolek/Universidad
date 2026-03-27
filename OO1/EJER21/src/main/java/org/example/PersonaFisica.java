package org.example;

public class PersonaFisica extends Cliente{

    private String nombre;
    private String direccion;
    private Integer dni;


    protected double aplicarDescuento(double total) {
        return total*0.10;
    }

}
