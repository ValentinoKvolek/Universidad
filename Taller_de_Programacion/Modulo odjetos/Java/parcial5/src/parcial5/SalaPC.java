/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial5;

/**
 *
 * @author valen
 */
public class SalaPC {
    
     private Pc [] vector;
     private int dimension;

    public SalaPC(int espacios) {
        this.dimension = espacios;
        this.vector =  new Pc[espacios];
        
        
    }
     
   public void agregarPc(int espacio , Pc pcRecibida){
        // segun el enunciado ya tegno asumido que espacio esta dentro de miu dimension fisica
        
       this.vector[espacio  - 1] = pcRecibida;
       
   
   }
   
   
   public void encederPc(){
       
       double min = 9999;
       
       int pcMin = 0;
        
       for (int i = 0 ; i < this.dimension; i ++ ){
           
    
           if(this.vector[i] != null){
               
               if( (this.vector[i].verEstado() != true) && (this.vector[i].medidorDeConsumo(min) == true) ) {
                   
                   pcMin = i;
  
               }
           }
       }
       
       this.vector[pcMin].encender();
   }    
   
    public String espaciosConPcON(){
        
        String aux =  "";
        
        for (int i = 0 ; i < this.dimension; i ++ ){
            
            if(this.vector[i] != null){
            
                if(this.vector[i].verEstado() == true ){
                
                   aux += " EL ESPACIO " + (i+ 1) + " TIENE UNA PC ENCENDIDA " + "\n";
                }
  
            }
        
        }
            
        return aux;
    }
      
}
