/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema3;

/**
 *
 * @author valen
 */
public class Triangulo {
    private double lado1;
    private double lado2;
    private double lado3;
    private String colorRelleneno;
    private String colorLinea;   
    private double Perimetro;
    private double Area;
    
    
    
    
    public Triangulo() {
        this.lado1 = lado1;
        this.lado2 = lado2;
        this.lado3 = lado3;
        this.colorRelleneno = colorRelleneno;
        this.colorLinea = colorLinea;
    }

    public double getLado1() {
        return lado1;
    }

    public void setLado1(double lado1) {
        this.lado1 = lado1;
    }

    public double getLado2() {
        return lado2;
    }

    public void setLado2(double lado2) {
        this.lado2 = lado2;
    }

    public double getLado3() {
        return lado3;
    }

    public void setLado3(double lado3) {
        this.lado3 = lado3;
    }

    public String getColorRelleneno() {
        return colorRelleneno;
    }

    public void setColorRelleneno(String colorRelleneno) {
        this.colorRelleneno = colorRelleneno;
    }

    public String getColorLinea() {
        return colorLinea;
    }

    public void setColorLinea(String colorLinea) {
        this.colorLinea = colorLinea;
    }

    public void setCalcularPerimetro(double lado1,double lado2,double lado3){
        this.Perimetro = lado1 + lado2 + lado3;
    }
    
    public double getCalcularPrimentro(){
        return Perimetro;
    }
    
    public void setCalcularArea(double lado1,double lado2,double lado3){
        double S;
        S = (lado1 + lado2 + lado3) / 2;
        this.Area =  Math.sqrt( S *(( S -lado1)*(S - lado2) * (S-lado3)));
    }
    public double getCalcularArea(){
        return Area;
    }
}


