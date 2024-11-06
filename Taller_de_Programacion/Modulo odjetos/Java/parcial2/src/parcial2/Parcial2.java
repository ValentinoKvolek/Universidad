/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial2;

/**
 *
 * @author valen
 */
public class Parcial2 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        // TODO code application logic here
        Torneo nuevoTorneo;
        
        
        
        nuevoTorneo = new Torneo(" FUT CHAMPION", 3, 2);
        
        
        Goleador nuevoGoleador;
        nuevoGoleador = new Goleador("valen", "river plate ", 50);
        Goleador nuevoGoleador2;
        nuevoGoleador2 = new Goleador("yoni", "BURUSSIA ", 40);
        Goleador nuevoGoleador3;
        nuevoGoleador3 = new Goleador("juanpi", " guatemala ", 2);
        Goleador nuevoGoleador4;
        nuevoGoleador4 = new Goleador("PELAO", "BOCA JUNTIORS ", 400);
        Goleador nuevoGoleador5;
        nuevoGoleador5 = new Goleador("THEO", "ESTUDIANTES ", 399);
        
        nuevoTorneo.agregarGoleador(1, nuevoGoleador4);
        nuevoTorneo.agregarGoleador(0, nuevoGoleador);
        nuevoTorneo.agregarGoleador(0, nuevoGoleador3);
        nuevoTorneo.agregarGoleador(2, nuevoGoleador2);
        nuevoTorneo.agregarGoleador(1, nuevoGoleador5);
        
        System.out.println(nuevoTorneo.goleadorConMenosGoles(0));
        System.out.println(nuevoTorneo.cantGoleadoresTotal());
        System.out.println(nuevoTorneo.toString());
        System.out.println("OTRA PRUEBA PARA ASEGURAR EL MIN :" + "\n");
        
        System.out.println(nuevoTorneo.goleadorConMenosGoles(1));
        
      
        
        
    }
    
}
