package tema2;
import PaqueteLectura.GeneradorAleatorio;
public class ejer3prac2 {
    
    public static void main(String[] args) {
        GeneradorAleatorio.iniciar();
        int dias = 5;
        int turnos = 8;
        Persona [][] matriz = new Persona [dias][turnos];
        Persona p;
        p = new Persona();
        System.out.println("ingresar edad");
        p.setEdad( GeneradorAleatorio.generarInt(101));      
        System.out.println("ingrese dni");
        p.setDNI(GeneradorAleatorio.generarInt(10000));
        System.out.println("ingresar nombre");
        p.setNombre(GeneradorAleatorio.generarString(6));
        
        int i = 0;
        while ((i < 5) && !(p.getNombre().equals("ZZZ"))){
            
            int j = 0;
            while ((j < 8) && !(p.getNombre().equals("ZZZ"))){
                
                matriz[i][j] = p;
                p = new Persona();
                System.out.println("ingresar edad");
                p.setEdad( GeneradorAleatorio.generarInt(101));      
                System.out.println("ingrese dni");
                p.setDNI(GeneradorAleatorio.generarInt(10000));
                System.out.println("ingresar nombre");
                p.setNombre(GeneradorAleatorio.generarString(6));
                
                j++;
            }
            i++;
        }
        for (int dia = 0; dia < dias; dia++) {
            for (int turno = 0; turno < turnos; turno++) {
                if (matriz[dia][turno] != null) { // Verificar que la persona no sea nula
                    System.out.println("Para el día " + (dia + 1) + " turno " + (turno + 1) + " la persona a entrevistar es " + matriz[dia][turno].getNombre());
                }
            }  
        }
    } 
}
