package org.example;

public class Esferas extends Pieza{
    public Esferas() {

    }

    double radio;

    @Override
    double getVolumen() {
        return (4.0 / 3.0) * Math.PI * Math.pow(radio, 3);
    }

    @Override
    double getSuperficie() {
        return 4 * Math.PI * Math.pow(radio, 2);
    }

}
