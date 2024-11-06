
package tema1;

//Paso 1: Importar la funcionalidad para lectura de datos
import PaqueteLectura.Lector;

public class Ej02Jugadores {

  
    public static void main(String[] args) {
        //Paso 2: Declarar la variable vector de double 
        int dmf =16;//0 a 5
        int cant=0;
        double sumaAlturas= 0;
        double [] vector = new double[dmf];
        double promedio = 0;
        
        for (int i =0; i<dmf ; i++){
            System.out.println("ingrese altura del jugador " + i);
            double altura = Lector.leerDouble();
            vector[i]= altura;
            sumaAlturas = sumaAlturas + altura;
        }
        
        promedio = sumaAlturas/16;
        
        
        for (int i =0; i<dmf ; i++)
            if(vector[i] > promedio)
                cant++;
                 
        System.out.println("la altura promedio de los jugadores fue de:  " + promedio);
        System.out.println("y la cantidad de jugadores que superan esa altura son:  " + cant);
    }
    
}
