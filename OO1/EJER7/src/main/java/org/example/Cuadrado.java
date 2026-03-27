package org.example;

public class Cuadrado implements Figura2D {

    private double lado;

    public void setLado(double valor){
        this.lado = valor;
    }
    public double getLado(){
        return this.lado;
    }
    public double getPerimetro(){
        return  lado * 4;
    }
    public double getArea(){
        return lado * lado;
    }


}
