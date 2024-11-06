/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package prac2prac5;

/**
 *
 * @author valen
 */
public class Prac2prac5 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        // Crear el estacionamiento con 3 pisos y 3 plazas por piso
        Estacionamiento estacionamiento = new Estacionamiento("Estacionamiento Central", "Calle Falsa 123", 5, 21, 3, 3);

        // Registrar 6 autos en el estacionamiento en distintos lugares
        estacionamiento.agregarAuto(new Auto("Juan", 1234), 1, 1);
        estacionamiento.agregarAuto(new Auto("Maria", 5678), 1, 2);
        estacionamiento.agregarAuto(new Auto("Pedro", 9101), 2, 1);
        estacionamiento.agregarAuto(new Auto("Ana", 1121), 2, 2);
        estacionamiento.agregarAuto(new Auto("Luis", 3141), 3, 1);
        estacionamiento.agregarAuto(new Auto("Sofia", 5161), 3, 2);

        // Mostrar la representación String del estacionamiento en consola
        estacionamiento.informarEstacionamiento();

        // Mostrar la cantidad de autos ubicados en la plaza 1
        System.out.println("Cantidad de autos en la plaza 1: " + estacionamiento.cantAutos(1));

        // Leer una patente por teclado e informar si dicho auto se encuentra en el estacionamiento o no
        System.out.print("Ingrese la patente a buscar: ");
        int patente = 3141;
        System.out.println(estacionamiento.buscarAuto(patente));
    }
    }
