/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial6;

/**
 *
 * @author valen
 */
public class Surtidor {
    
    private boolean servicio;
    
    private int dimf ;
    
    private int diml =0;
    
    private Venta [] vectorVentas;

    public Surtidor( int maxVentas) {
        
        this.servicio = true;
        this.dimf = maxVentas;
        this.vectorVentas = new Venta[maxVentas];
        
    }
    
    
   public void AgregarVentas(int dniNuevo ,double precioxM3 ,double m3, String pago){
       
       Venta NuevaVenta;
       
       NuevaVenta = new Venta(dniNuevo,precioxM3,m3,pago);
       
       if (diml < dimf){}{
       
        this.vectorVentas[diml]=NuevaVenta;
        diml++;
                
       }
        
    }
   
   
    public double total (){

        double total= 0;

        for (int i = 0;  i < diml; i++){

            total += this.vectorVentas[i].obternerm3();

        }

        return total;
    }
      

    public void cambiarSituacion(){

        if(this.servicio = true){

        this.servicio = false;
        }
        else
            this.servicio = true;

    }
    
    public Venta BuscarMax(){
        
        Venta auxVenta;
        
        double max =-99; 
        
        for (int i = 0;  i < diml; i++){
        
            if(this.vectorVentas[i].obternerMonto() > max){
            
                max  = this.vectorVentas[i].obternerMonto();
                auxVenta = new Venta(todo loq ue necesita la venta);
            }
        
        }
        
        return auxVenta;
    
    }
    
    
    public String toString(int pos){
   
        String aux;
        
        aux = "Surtidor : " + ( pos + 1 ) + " estado: " + this.servicio + "\n";
        
        for (int i = 0;  i < diml; i++){
        
            aux += this.vectorVentas[i].toString();
        }
        
       return aux;
                
    
    }
    
    
}
