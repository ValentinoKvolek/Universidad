/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial9;

/**
 *
 * @author valen
 */
public abstract class Compra  {
    
    private int nroCompra;
    private int cantMaxComprados;
    private int diml= 0;
    private Producto[] vector;

    public Compra(int nroCompra, int cantMaxComprados) {
        this.nroCompra = nroCompra;
        this.cantMaxComprados = cantMaxComprados;   
        this.vector = new Producto[cantMaxComprados];
    }
    
    public void agregarCompra(Producto nuevoProducto){
        
        this.vector[diml]= nuevoProducto; //tengo que asumir que hay espacio disponible segun enunciado
        diml ++;
    
    }
    
    
   public double totalPrecio(){
   
       double total =0;
       
       for (int i =0;  i<diml; i++){
           
           if(this.vector[i] != null){
       
                total += this.vector[i].obtenerPrecio();
            
           }
       } 
       
       return (total*1.21);
   }
   
   
   public String toString(){
   
       String aux;
       
       aux =  " nro de compra : " + this.nroCompra + "\n" +
               "productos : ";
       
       for (int i =0; i < diml; i++){
           
           if(this.vector[i] != null){
           
                aux += this.vector[i].toString() + "\n";
           }
       }
      
       return aux; 
       
   }
    
    
    
}
