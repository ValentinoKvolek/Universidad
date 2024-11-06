/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema3;
import PaqueteLectura.Lector;

/**
 *
 * @author valen
 */
public class ejer1prac3 {
       
      public static void main(String[] args) {
          Triangulo Nuevotriangulo;
          Nuevotriangulo = new Triangulo();
          System.out.println("ingrese lado 1: ");
          Nuevotriangulo.setLado1(Lector.leerDouble());
          System.out.println("ingrese lado 2: ");
          Nuevotriangulo.setLado2(Lector.leerDouble());
          System.out.println("ingrese lado 3: ");
          Nuevotriangulo.setLado3(Lector.leerDouble());
          System.out.println("ingrese color de relleno: ");
          Nuevotriangulo.setColorRelleneno(Lector.leerString());
          System.out.println("ingrese color de linea: ");
          Nuevotriangulo.setColorLinea(Lector.leerString());
          
          Nuevotriangulo.setCalcularPerimetro(Nuevotriangulo.getLado1(), Nuevotriangulo.getLado2(), Nuevotriangulo.getLado3());
          
          Nuevotriangulo.setCalcularArea(Nuevotriangulo.getLado1(), Nuevotriangulo.getLado2(), Nuevotriangulo.getLado3());
          
         
          
          System.out.println("el perimeto del triangulo es de :  " + Nuevotriangulo.getCalcularPrimentro());
          System.out.println("el area del triangulo es : "  + Nuevotriangulo.getCalcularArea());
          
    }
    
}
