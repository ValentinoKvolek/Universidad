package org.example;

public class PrismasRectangulares extends Pieza {

    double ladoMayor;
    double ladoMenor;
    double altura;

    public PrismasRectangulares() {

    }

    @Override
    double getVolumen() {
        return ladoMayor * ladoMenor * altura;
    }

    @Override
    double getSuperficie() {
        return 2 * (ladoMayor * ladoMenor + ladoMayor * altura + ladoMenor * altura);
    }
}
