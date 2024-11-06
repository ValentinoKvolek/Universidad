/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema2;
import PaqueteLectura.Lector;
/**
 *
 * @author Valentino kvolek
 */
public class ejer5prac2 {
    public static void main(String[] args) {
         Partido [] vector = new Partido [20];
         Partido p;
         Boolean CantidadPatidos = false;
         int DIMF =0;
         int contadorPartidos= 0;
         
         p = new Partido();
         System.out.println("ingrese el nombre del equipo local: ");
         p.setLocal(Lector.leerString());
         System.out.println("ingresar golea del equipo Local: ");
         p.setGolesLocal(Lector.leerInt());
         System.out.println("ingrese el nombre del equipo visitante: ");
         p.setVisitante(Lector.leerString());
         System.out.println("ingresar los goles del equipo visitante: ");
         p.setGolesVisitante(Lector.leerInt());
         
         while( !(p.getVisitante().equals("ZZZ"))&&!(CantidadPatidos)){
             
                vector[contadorPartidos] = p;
                contadorPartidos++;
                DIMF ++;
                p = new Partido();
                System.out.println("ingrese el nombre del equipo local: ");
                p.setLocal(Lector.leerString());
                System.out.println("ingresar golea del equipo Local: ");
                p.setGolesLocal(Lector.leerInt());
                System.out.println("ingrese el nombre del equipo visitante: ");
                p.setVisitante(Lector.leerString());
                System.out.println("ingresar los goles del equipo visitante: ");
                p.setGolesVisitante(Lector.leerInt());
                
                if(contadorPartidos == 20){
                    CantidadPatidos = true;
                }
                
         }
         int cantRiver=0;
         int cantGolesBoca =0;
         for (int i=0; i < DIMF; i++){
             System.out.println("EQUIPO-LOCAL: " + vector[i].getGolesLocal() + " EQUIPO-VISITANTE: " + vector[i].getGolesVisitante());

                if(vector[i].getLocal().equals("river")){

                    if(vector[i].getGolesLocal() > vector[i].getGolesVisitante()){
                        cantRiver++;
                    }
               }

               if(vector[i].getVisitante().equals("river")){
                    if(vector[i].getGolesVisitante() > vector[i].getGolesLocal()){
                        cantRiver++;
                    }
                }

                if (vector[i].getLocal().equals("boca")){
                    cantGolesBoca += vector[i].getGolesLocal();
                }
        }

         System.out.println("river gano : " + cantRiver + " partidos.");
         System.out.println("boca metio: " + cantGolesBoca + " goles.");
    }
}
