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
public class Circulo {
    
    private double radio;
    private String ColorRelleno;
    private String ColorLinea;

    public Circulo(double radio, String ColorRelleno, String ColorLinea) {
        this.radio = radio;
        this.ColorRelleno = ColorRelleno;
        this.ColorLinea = ColorLinea;
    }

    public double getRadio() {
        return radio;
    }

    public void setRadio(double radio) {
        this.radio = radio;
    }

    public String getColorRelleno() {
        return ColorRelleno;
    }

    public void setColorRelleno(String ColorRelleno) {
        this.ColorRelleno = ColorRelleno;
    }

    public String getColorLinea() {
        return ColorLinea;
    }

    public void setColorLinea(String ColorLinea) {
        this.ColorLinea = ColorLinea;
    }
    
    public double calcularPerimetro(double radio){
        
        double perimetro = ((2 * Math.PI) * radio);
        
        return perimetro ;
    }
    
   public double calcularArea(double radio){
       double area = ((Math.PI)* radio * radio);
       
       return area;
   }

    @Override
    public String toString() {
        return "Circulo{" + "radio=" + radio + ", ColorRelleno=" + ColorRelleno + ", ColorLinea=" + ColorLinea + '}';
    }
   
}
