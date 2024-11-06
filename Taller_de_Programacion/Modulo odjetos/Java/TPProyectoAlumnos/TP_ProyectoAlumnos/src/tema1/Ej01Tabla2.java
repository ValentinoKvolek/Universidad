
package tema1;
import PaqueteLectura.GeneradorAleatorio;

public class Ej01Tabla2 {

    public static void main(String[] args) {
        
        GeneradorAleatorio.iniciar();
        int DF=12;  // indices de 0 a 11
        int numero;
        int [] tabla2 = new int[DF]; // indices de 0 a 11
        for (int i=0; i<DF ;i++) 
            tabla2[i]=2*i; //carga vector con multiplos de 2
        numero = GeneradorAleatorio.generarInt(12);// genera numeros aleatorios del 0 al 11
        
        while ( numero !=11){  // corte de control!
        numero = GeneradorAleatorio.generarInt(12);
        System.out.println("2x" + numero + "="+ tabla2[numero]);
        }
    }
    
}
