package tema2;
import PaqueteLectura.GeneradorAleatorio;
import PaqueteLectura.Lector;
public class ejer4prac2 {
    
    public static void main(String[] args) {
        GeneradorAleatorio.iniciar();
        int dias = 5;
        int turnos = 8;
        int contadorTotal= 0;
        int diaDeseado;
        boolean continuar = true;
       
        Persona [][] matriz = new Persona [dias][turnos];
        
        int [] vector = new int [5];
        for (int i = 0; i < 5; i++){
            vector[i] = 0;
        }
        
        Persona p;
        
        while (contadorTotal < 40 && continuar) {
            
            p = new Persona();
            System.out.println("ingresar edad");
            p.setEdad(GeneradorAleatorio.generarInt(101));      
            System.out.println("ingrese dni");
            p.setDNI(GeneradorAleatorio.generarInt(10000));
            System.out.println("ingresar nombre");
            p.setNombre(Lector.leerString());
            
            if (p.getNombre().equals("ZZZ")) {
                continuar = false; 
            }else {
                
                
                System.out.println("ingresar dia que quiere el turno");
                diaDeseado = Lector.leerInt();
                
                if (diaDeseado >= 0 && diaDeseado < dias) {
                    boolean turnoAsignado = false;
                    
              
                    for (int j = 0; j < turnos && !turnoAsignado; j++) {
                        if (matriz[diaDeseado][j] == null) {
                            matriz[diaDeseado][j] = p;
                            vector[diaDeseado]++;
                            contadorTotal++;
                            turnoAsignado = true;
                            
                        }
                    }
                    
                    if (!turnoAsignado) {
                        System.out.println("El día " + diaDeseado + " está lleno!");
                    }
                } 
            }
        }

        for (int dia = 0; dia < dias; dia++) {
            for (int turno = 0; turno < turnos; turno++) {
                if (matriz[dia][turno] != null) { // Verificar que la persona no sea nula
                    System.out.println("Para el día:  " + (dia + 1) + " turno:  " + (turno + 1) + " la persona a entrevistar es:  " + matriz[dia][turno].getNombre());
                    
                }
            }  
            System.out.println("la cantidad del dia " + (dia + 1)+ "es de " + vector[dia]);
        }
    } 
}
