package org.example;


public class Circulo implements Figura2D {

    private double radio;

    public void setDiametro(double valor) {
        this.radio = valor / 2;
    }

    public double getDiametro() {
        return this.radio * 2;
    }

    public void setRadio(double valor) {
        this.radio = valor;
    }

    public double getRadio() {
        return this.radio;
    }

    public double getPerimetro() {
        return 2 * Math.PI * this.radio;
    }

    public double getArea() {
        return Math.PI * (this.radio * this.radio);
    }



}
