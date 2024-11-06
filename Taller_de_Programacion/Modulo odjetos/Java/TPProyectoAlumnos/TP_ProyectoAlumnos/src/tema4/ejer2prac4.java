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
public class ejer2prac4 {
    public static void main(String[] args) {
        Jugador nuevoJugador;
        nuevoJugador = new Jugador(100,200,"Lionel",120.000,20);
        Entrenador nuevoEntrenador;
        nuevoEntrenador =  new Entrenador(11,"guardiola",10.000,10);
        System.out.println(nuevoJugador);
        System.out.println(nuevoEntrenador);
    }
}
