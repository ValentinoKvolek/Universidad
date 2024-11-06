/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package ejer4prac5;

/**
 *
 * @author valen
 */
public class Ejer4prac5 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
    
 Director director = new Director("Juan Pérez", 12345678, 5);

        // Coro semicircular (desordenado)
        Csemicircular coroSemicircular = new Csemicircular("Coro Semicircular", director, null, 5);
        coroSemicircular.agregarCorista(new Corista("Corista 1", 1000001, 20, 100)); // tono 100
        coroSemicircular.agregarCorista(new Corista("Corista 2", 1000002, 21, 120)); // tono 120
        coroSemicircular.agregarCorista(new Corista("Corista 3", 1000003, 22, 90));  // tono 90 (desordenado)
        coroSemicircular.agregarCorista(new Corista("Corista 4", 1000004, 23, 150)); // tono 150
        coroSemicircular.agregarCorista(new Corista("Corista 5", 1000005, 24, 200)); // tono 200

        // Coro por hileras (cumpliendo las condiciones)
        Chileras coroHileras = new Chileras("Coro por Hileras", director, null, 3);
        coroHileras.agregarCorista(new Corista("Corista 1", 2000001, 20, 140)); // tono 140
        coroHileras.agregarCorista(new Corista("Corista 2", 2000002, 21, 140)); // tono 140
        coroHileras.agregarCorista(new Corista("Corista 3", 2000003, 22, 140)); // tono 140

        coroHileras.agregarCorista(new Corista("Corista 4", 3000001, 23, 130)); // tono 130
        coroHileras.agregarCorista(new Corista("Corista 5", 3000002, 24, 130)); // tono 130
        coroHileras.agregarCorista(new Corista("Corista 6", 3000003, 25, 130)); // tono 130

        coroHileras.agregarCorista(new Corista("Corista 7", 4000001, 26, 120)); // tono 120
        coroHileras.agregarCorista(new Corista("Corista 8", 4000002, 27, 120)); // tono 120
        coroHileras.agregarCorista(new Corista("Corista 9", 4000003, 28, 120)); // tono 120

        // Informar sobre los coros
        System.out.println("\nCoro Semicircular:");
        coroSemicircular.informar();
        System.out.println("¿Está bien formado? " + (coroSemicircular.ordenado() ? "Sí" : "No"));

        System.out.println("\nCoro por Hileras:");
        coroHileras.informar();
        System.out.println("¿Está bien formado? " + (coroHileras.ordenado() ? "Sí" : "No"));
    }
    
}
