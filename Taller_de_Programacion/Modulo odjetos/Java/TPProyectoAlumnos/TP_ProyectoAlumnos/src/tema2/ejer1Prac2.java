/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema2;

import PaqueteLectura.Lector;
/**
 *
 * @author valen
 */
public class ejer1Prac2 {
        public static void main(String[] args) {
            System.out.println("ingresar nombre");
            String nombreNuevo=Lector.leerString();
            System.out.println("ingresar dni");
            int dniNuevo=Lector.leerInt();
            System.out.println("ingresar edad");
            int edadNuevo=Lector.leerInt();
            Persona NuevaPersona = new Persona(nombreNuevo,dniNuevo,edadNuevo);
            System.out.println(NuevaPersona.toString());
        }
}
        
