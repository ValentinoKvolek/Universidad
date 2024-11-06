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
public class ejer5prac3 {
      
    public static void main(String[] args) {
        
        Circulo nuevoCirculo;
        
        System.out.println("ingresa radio del circulo");
        double radioNuevo = Lector.leerDouble();
        System.out.println("ingrese el color de relleno del circulo");
        String colorRellenoNuevo = Lector.leerString();
        System.out.println("ingrese el color de linea del circulo");
        String colorLineaNuevo = Lector.leerString();
        
        
        nuevoCirculo = new Circulo(radioNuevo,colorRellenoNuevo,colorLineaNuevo);
        
        System.out.println(nuevoCirculo);
        
        System.out.println("el perimetro es : " + nuevoCirculo.calcularPerimetro(radioNuevo) );
        
        System.out.println("el area es : " + nuevoCirculo.calcularArea(radioNuevo) );
    }
    
    

    
    
}
