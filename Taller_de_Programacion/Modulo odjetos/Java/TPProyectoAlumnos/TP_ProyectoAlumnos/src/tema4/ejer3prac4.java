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
public class ejer3prac4 {
     public static void main(String[] args) {
         Persona nuevaPersona;
         nuevaPersona = new Persona("valen", 4284984, 23);
         Trabajador nuevoTrabajador;
         nuevoTrabajador = new Trabajador("programador","yoni", 23456543, 21);
         
         
         System.out.println(nuevaPersona);
         System.out.println(nuevoTrabajador);
    }
}
