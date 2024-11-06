/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema1;

//Paso 1. importar la funcionalidad para generar datos aleatorios
import PaqueteLectura.GeneradorAleatorio;
import PaqueteLectura.Lector;

public class Ej03Matrices {

    public static void main(String[] args) {
        
	    //Paso 2. iniciar el generador aleatorio     
            GeneradorAleatorio.iniciar();
            int dimf =5;
            int dimc =5;
            int [] vector = new int [dimc];

           //Paso 3. definir la matriz de enteros de 5x5 e iniciarla con nros. aleatorios 
           int [][] matriz = new int [dimf][dimc];
           int i;
           int j; 

           for (i=0;i<dimf; i++)
               for(j=0;j<dimc; j++)
                   matriz[i][j]= GeneradorAleatorio.generarInt(31);

           //Paso 4. mostrar el contenido de la matriz en consola
           for (i=0; i<dimf; i++)
                for (j=0;j<dimc; j++)
                   System.out.println("La Pos: " + i + "><" + j +  " tiene: " + matriz[i][j]);


           //Paso 5. calcular e informar la suma de los elementos de la fila 1
           int Suma = 0;
               for (j=0; j < dimc; j++)
                   Suma += matriz[1][j];

           System.out.println("la suma de todos los elementos de la fila 1 : " + Suma);

           //Paso 6. generar un vector de 5 posiciones donde cada posición j contiene la suma de los elementos de la columna j de la matriz. 
           //        Luego, imprima el vector.

           for (j=0;j<dimc; j++){
               
               int SumaParaVector =0;
                   for (i=0;i<dimf; i++){
                       SumaParaVector += matriz[i][j];
                   }
               vector[j] = SumaParaVector;
           }
           
           for (j=0;j<dimc;j++){
              System.out.println("La suma de todos los elementos de la columna " + j + " es: " + vector[j]);
           }
           
           //Paso 7. lea un valor entero e indique si se encuentra o no en la matriz. En caso de encontrarse indique su ubicación (fila y columna)
           //   y en caso contrario imprima "No se encontró el elemento".
           
           System.out.println("ingrese valor para buscar en la matriz: ");
           int ValorNuevo = Lector.leerInt();
           int fila = 0;
           int colum = 0;
           boolean encontro = false;
           
           
           for (i = 0 ; i < dimf ; i++)
               for(j=0 ; j < dimc ; j++)
                   if(ValorNuevo == matriz[i][j]){
                       fila = i;
                       colum = j;
                       encontro = true;
                   }
           if (encontro == true)
                System.out.println("la fila fue: " + fila + " y la columna: " + colum);
           else{
                System.out.println("no se encontro");
           }
              
           
    }
}
