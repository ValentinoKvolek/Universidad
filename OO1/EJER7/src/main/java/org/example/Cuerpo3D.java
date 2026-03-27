package org.example;

public class Cuerpo3D extends Object {

    private double altura;

    private Figura2D caraBasal;

    public void setAltura(Double valor){
        this.altura = valor;
    }
    public double getAltura(){
        return altura;
    }
    public void setCaraBasal(Figura2D cara){
        this.caraBasal = cara;
    }
    public double getVolumen(){
        return  caraBasal.getArea() * altura;
    }
    public double getSuperficieExterior(){
        return  (2 * caraBasal.getArea()) + caraBasal.getPerimetro() * altura;
    }
}
