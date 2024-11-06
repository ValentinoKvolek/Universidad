/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema2;

import PaqueteLectura.GeneradorAleatorio;
/**
 *
 * @author Valentino kvolek
 */
public class ejer2prac2 {
    public static void main(String[] args) {
        Persona p;
        int diml = 0;
        Persona [] vector = new Persona[16];
        p = new Persona();
        System.out.println("ingresar edad");
        p.setEdad( GeneradorAleatorio.generarInt(101));      
        System.out.println("ingrese dni");
        p.setDNI(GeneradorAleatorio.generarInt(10000));
        System.out.println("ingresar nombre");
        p.setNombre(GeneradorAleatorio.generarString(6));
        int pos = 0;
        while(p.getEdad() != 0 && pos <15) {
            vector[pos] = p;
            pos ++;
            diml ++;
            p = new Persona();
            System.out.println("ingresar edad");
            p.setEdad( GeneradorAleatorio.generarInt(101));      
            System.out.println("ingrese dni");
            p.setDNI(GeneradorAleatorio.generarInt(10000));
            System.out.println("ingresar nombre");
            p.setNombre(GeneradorAleatorio.generarString(6));
        }
        int cantMayores = 0;
        for (int i=0; i < diml; i ++){
            if (vector[i].getEdad() > 65){
                cantMayores ++;
            } 
        }
        
        Persona Pn = vector[0];
        for (int i = 1; i < diml; i++) {
            if (vector[i].getDNI() < Pn.getDNI()) {
                    Pn = vector[i];
            }
         }
          
        System.out.println("la cantidad de personas mayores a 65 son: "+ cantMayores);
        System.out.println(Pn.toString());
    }
}
