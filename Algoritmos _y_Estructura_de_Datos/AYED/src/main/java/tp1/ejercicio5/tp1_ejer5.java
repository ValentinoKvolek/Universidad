/*
5. Dado un arreglo de valores tipo entero se desea calcular el valor máximo, mínimo, y promedio
en un único métod. Escriba tres métodos de clase, donde respectivamente:

a. Devuelva lo pedido por el mecanismo de retorno de un métod en Java ("return").

b. Devuelva lo pedido interactuando con algún parámetro (el parámetro no puede ser de
tipo arreglo).

c. Devuelva lo pedido sin usar parámetros ni la sentencia "return

 */
package tp1.ejercicio5;

import java.util.Arrays;

public class tp1_ejer5 {

    public static String calcular(int[]array){
        String aux;
        int max = Arrays.stream(array).max().orElseThrow(() -> new IllegalArgumentException("El array está vacío"));
        // esto ser hace por que el stream devuelve un objeto tipo OptionalInt para evitar el error del array vacio.
        int min = Arrays.stream(array).min().orElseThrow(() -> new IllegalArgumentException("El array está vacío"));
        int suma = 0;
        for(int i =0; i<array.length; i++){
            suma = suma + array[i];
        }

        int prom = suma/array.length;
        aux =("el maximo numero del vector es : " + max + '\'' + "el minimo es : "+ min + '\'' +"y el promedio es: "+ prom);
        return aux;

    }

    public static void calcularByC(Cofre cofre,int[]array){
        int max = Arrays.stream(array).max().orElseThrow(() -> new IllegalArgumentException("El array está vacío"));
        int min = Arrays.stream(array).min().orElseThrow(() -> new IllegalArgumentException("El array está vacío"));
        int suma =0;
        for(int i =0; i<array.length; i++){
            suma = suma + array[i];
        }

        int prom = suma/array.length;
        cofre.setMax(max);
        cofre.setMin(min);
        cofre.setProm(prom);
    }

    public static void main(String[] args) {
        int[] array = new int[5];
        Cofre cofre = new Cofre();
        array[0] = 70;
        array[1] = 1;
        array[2] = 13548;
        array[3] = 4;
        array[4] = -5;

        System.out.println(calcular(array));
        calcularByC(cofre,array);
        System.out.printf("el maximo sin utlizar return es : "+ cofre.getMax()+ "el minimo" + cofre.getMin()+" y el promedio : "+ cofre.getProm());
    }

}
