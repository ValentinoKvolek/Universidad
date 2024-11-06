/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial8;

/**
 *
 * @author valen
 */
public class Sistema {
    
    private Paciente [][] matrizAgenda;

    public Sistema() {
        
        this.matrizAgenda = new Paciente[5][6]; // asumo que la matriz por defecto ya esta sin pacientes por que java inicaliza en null;
        
    }
    
    
    public void agendarPaciente(Paciente p, int dia, int turno){ // T. Asuma que el turno está libre. D y T son válidos
    
        this.matrizAgenda[dia - 1 ][turno - 1 ] =  p; //me guardo el paciente.
      
    } 
    
    
    
    public String liberarAgenda(int dia){
        
        String aux = "";
        
        for (int i = 0 ; i < 6; i++ ){ //recorro todos lso turnos del dia ingresado asumiendo que es valido.
            
            if(this.matrizAgenda[dia - 1][i] != null){
                
                aux +=matrizAgenda[dia - 1][i].informarPaciente() + "\n";
                 matrizAgenda[dia - 1][i]=  null;
                 
            }
        }
        
        return aux;
    
    }
    
    
    public double calcularCostoSemanal(){
    
        double total= 0;
        
        
        for (int i = 0 ; i < 5; i++){
            
            for (int j = 0 ; j < 6; j++){
                
                if(this.matrizAgenda[i][j] != null){
            
                    total +=  this.matrizAgenda[i][j].obtenerCosto();
                    
                }
            }
        
        }
    
        return total;
    }
    
    
}
