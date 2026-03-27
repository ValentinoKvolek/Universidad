package org.example;

abstract class Pieza {
    String material;
    String color;

    abstract double getVolumen();
    abstract double getSuperficie();

    public String getMaterial() {
        return material;
    }

    public String getColor() {
        return color;
    }
}
