/*
5- El dueño de un restaurante entrevista a cinco clientes y les pide que califiquen
(con puntaje de 1 a 10) los siguientes aspectos: (0) Atención al cliente (1) Calidad
de la comida (2) Precio (3) Ambiente.
Escriba un programa que lea desde teclado las calificaciones de los cinco clientes
para cada uno de los aspectos y almacene la información en una estructura. Luego
imprima la calificación promedio obtenida por cada aspecto.
 */
package tema1;

import PaqueteLectura.Lector;
/**
 *
 * @author valen
 */
public class ejer5prac1 {
    public static void main(String[] args) {
        int [][] matriz = new int[5][4];
        int [] vector = new int [4];
        for (int i=0; i<5; i++){
             for (int j=0; j<4; j++){
                 System.out.println("ingrese puntaje: ");
                 int puntaje = Lector.leerInt();
                 matriz[i][j]+=puntaje;
                 vector[j]+=puntaje;
             }
        }
        for (int i=0; i<4; i++)
             System.out.println("el punraje promedio en el aspecto " + i + " fue de " + vector[i]/5);
    }
    
}
