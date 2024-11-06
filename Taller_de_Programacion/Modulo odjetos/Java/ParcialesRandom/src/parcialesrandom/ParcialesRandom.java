/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcialesrandom;


import PaqueteLectura.GeneradorAleatorio;

/**
 *
 * @author valen
 */
public class ParcialesRandom {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        
        GeneradorAleatorio.iniciar();
        
        
        int parcialRamdon = GeneradorAleatorio.generarInt(23);
        
        System.out.println("EL PARCIAL QUE TOCA HACER ES EL:" + parcialRamdon);
        
    }
    
}
