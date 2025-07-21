/*
2. Escriba un métod de clase que dado un número n devuelva un nuevo arreglo de tamaño n
con los n primeros múltiplos enteros de n mayores o iguales que 1.

Ejemplo: f(5) = [5; 10; 15; 20; 25]; f(k) = {n*k donde k : 1..k}

Agregue al programa la posibilidad de probar con distintos valores de n ingresandolos por
teclado, mediante el uso de System.in. La clase Scanner permite leer de forma sencilla
valores de entrada.
 */

package tp1.ejercicio2;

import java.util.Scanner;

public class tp1_ejer2 {

    public static void crearVector(int n){
        int[] array = new int[n];
        int pos = 0;
        int multiplo =1;
        while (pos < array.length){
            array[pos] =(n * multiplo);
            System.out.println(array[pos]);
            pos ++;
            multiplo ++;
        }
    }
    public static void main(String[] args) {

        Scanner scanner = new Scanner(System.in);
        System.out.print("Ingrese n o ingrese 0 para terminar.");
        int n = scanner.nextInt();

        while (n != 0 ){
            crearVector(n);
            System.out.print("Ingrese n o ingrese 0 para terminar.");
            n = scanner.nextInt();
        }
    }
}
