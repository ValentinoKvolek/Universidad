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
public class Pc {
    
    private boolean estado;
    private double consumoXhora;

    public Pc( int consumo) {
        
        this.estado = false;
        this.consumoXhora = consumo;
        
    }
    
    public boolean medidorDeConsumo(double min){
        
        boolean aux = false;
        
        if(this.consumoXhora < min){
            
            min = this.consumoXhora;
            aux = true;
            
        }
                
        return aux;     
        
    
    }
    
    public double obtenerConsumo(){
    
        return this.consumoXhora;
    
    }
    
    public void encender(){
    
        this.estado = true;
    }
    
    public boolean verEstado(){
    
        return this.estado;
    }

}
