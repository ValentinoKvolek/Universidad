/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema4;

/**
 *
 * @author valen
 */
public class Triangulo extends Figura {
    
    private double lado1 , lado2 , lado3;

    public Triangulo(double lado1, double lado2, double lado3, String unCR, String unCL) {
        super(unCR, unCL);
        this.setLado1(lado1);
        this.setLado2(lado2);
        this.setLado3(lado3);
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
    
    
    public double calcularPerimetro(){
        
        double perimetro = lado1 + lado2 + lado3;
        
        return perimetro;
        
    }
    
    public double calcularArea(){
        
         double semiperimetro = (this.calcularPerimetro()) / 2;
         double area = Math.sqrt(semiperimetro * (semiperimetro - lado1) * (semiperimetro - lado2) * (semiperimetro - lado3));
         
         return area;
    }
    
    public String toString(){
       String aux = super.toString() + 
                    " Lado 1 " + this.getLado1() +
                    " Lado 2 " + this.getLado2() +
                    " Lado 3 " + this.getLado3() ;
                    
       return aux;
    }
    
}
