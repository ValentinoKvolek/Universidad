/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial3;

/**
 *
 * @author valen
 */
public class Butaca {
    
    private String Descriptor = " butaca ";
    
    private double precio = 100 + 800;
    
    private boolean estado = false;

    
    public Butaca(){}
   
    public void cambiarEstado(){
        
        if(this.estado == true ){
            
             this.estado = false;
        }
        
        else this.estado= true;
    }
    
    
    public double precioButaca(int fila){
        this.precio += this.precio *fila;
        return (precio);
        
    }
    
    public String toString(int i, int b){
        
        String aux;
        aux = "descriptor : " + this.Descriptor + i + b +
                " precio : " +  this.precio +
                    "estado" + this.estado;
        return aux;
                
    
    }
    
}
