/*
4- Un edificio de oficinas está conformado por 8 pisos (1..8) y 4 oficinas por piso
(1..4). Realice un programa que permita informar la cantidad de personas que
concurrieron a cada oficina de cada piso. Para esto, simule la llegada de personas al
edificio de la siguiente manera: a cada persona se le pide el nro. de piso y nro. de
oficina a la cual quiere concurrir. La llegada de personas finaliza al indicar un nro.
de piso 9. Al finalizar la llegada de personas, informe lo pedido.
 */
package tema1;

import PaqueteLectura.Lector;

/**
 *
 * @author valen
 */
public class ejer4prac1 {
    
    public static void main(String[] args) {
        
        int oficinas =4; // 0 a 4
        int pisos =8; // 0 a 8
        int [][] matriz = new int [pisos][oficinas];
        System.out.println("Ingrese nro piso: ");
        int nroPiso = Lector.leerInt();
   
        while(nroPiso != 9){

            System.out.println("Ingrese nro oficia: ");
            int nroOficina = Lector.leerInt();

            matriz[nroPiso-1][nroOficina-1]+=1;

            System.out.println("Ingrese nro piso: ");
            nroPiso = Lector.leerInt();
        }

        for (int i=0; i < pisos; i++ ){
            for (int j=0; j < oficinas; j++){
                System.out.println("piso: " + (i+1) + " oficia :" + (j+1) + " la cantidad es : " + matriz[i][j]);            
            } 
        }

        
    }
    
}
