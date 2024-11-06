/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package ejer3prac5;

/**
 *
 * @author valen
 */
public class Ejer3prac5 {

    /**
     * @param args the command line arguments
     */
   public static void main(String[] args) {
        // Crear un evento ocasional
        EventoOcasional evento = new EventoOcasional("TV", "Juan", 10, "La Banda", 5);
        evento.agregarTema("Canción 1");
        evento.agregarTema("Canción 2");
        evento.agregarTema("Canción 3");

        // Imprimir costo y actuar
        System.out.println("Costo del Evento Ocasional: " + evento.calcularCosto());
        System.out.println(evento.actuar());

        // Crear fechas para la gira
        Fecha[] fechas = new Fecha[3]; 
        fechas[0] = new Fecha("2024-10-01", "Ciudad A");
        fechas[1] = new Fecha("2024-10-02", "Ciudad B");
        fechas[2] = new Fecha("2024-10-03", "Ciudad C");

        // Crear una gira
        Gira gira = new Gira("Gira de Verano", fechas, "La Banda", 5, 3);
        gira.agregarTema("Canción A");
        gira.agregarTema("Canción B");
        gira.agregarTema("Canción C");

        // Agregar fechas a la gira
        for (Fecha fecha : fechas) {
            gira.agregarFecha(fecha);
        }

        // Imprimir costo y actuar
        System.out.println("Costo de la Gira: " + gira.calcularCosto());
        System.out.println(gira.actuar());
    }
}

