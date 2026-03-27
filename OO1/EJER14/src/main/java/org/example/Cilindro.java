package org.example;

public class Cilindro extends Pieza {
    double radio;
    double altura;

    public Cilindro() {

    }

    @Override
    double getVolumen() {
        return Math.PI * Math.pow(radio, 2) * altura;
    }

    @Override
    double getSuperficie() {

        return 2 * Math.PI * radio * altura + 2 * Math.PI * Math.pow(radio, 2);
    }
}
