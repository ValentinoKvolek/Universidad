
package practicandoconjava1;
import PaqueteLectura.Lector;
import PaqueteLectura.GeneradorAleatorio;

public class PracticandoConJava1 {

    public static void main(String[] args) {
        GeneradorAleatorio.iniciar();
        int DF=11;  
        int numero= 0;
        int [] tabla2 = new int[DF];
        for (int i=0;i<DF;i++) 
            tabla2[i]=2*i;
        
        while(numero != 11){
            
            System.out.print("se genero un numero aleatorio: ");
            numero = GeneradorAleatorio.generarInt(12);
            System.out.println("2x" + numero + "="+ tabla2[numero]); 
        }
    }
    
}
