/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema4;
import PaqueteLectura.Lector;
/**
 *
 * @author valen
 */
public class ejer1prac4 {
    public static void main(String[] args) {
        System.out.println("ingresar lado 1 del triangulo");
        double Nuevolado =Lector.leerDouble();
        System.out.println("ingresar lado 2 del triangulo");
        double Nuevolado2 =Lector.leerDouble();
        System.out.println("ingresar lado 3 del triangulo");
        double Nuevolado3 =Lector.leerDouble();
        System.out.println("ingrese color de linea");
        String relleno = Lector.leerString();
        System.out.println("ingrese color de relleno");
        String linea = Lector.leerString();
        
        Triangulo NuevoTriangulo;
         Circulo nuevoCirculo;
        NuevoTriangulo = new Triangulo(Nuevolado,Nuevolado2,Nuevolado3,relleno,linea);
        nuevoCirculo =  new Circulo(40,"rojo", "azul");
        System.out.println(NuevoTriangulo);
        System.out.println(nuevoCirculo);
        NuevoTriangulo.despintar();
        nuevoCirculo.despintar();
        System.out.println(NuevoTriangulo);
        System.out.println(nuevoCirculo);
    }
    
}
