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
public class Estacion {
    
    private String direccion;
    
    private int MaximoVentas;
    
    private double preciom3;
    
    Surtidor [] surtidores = new Surtidor [6];

    public Estacion( String direccion, double preciom3, int maxVentasXsurtidor) {
        
        this.direccion = direccion;
        
        this.MaximoVentas = maxVentasXsurtidor;
        
        this.preciom3 = preciom3;
        
        for (int i = 0;  i <  6; i++ ){
        
            this.surtidores[i] = new Surtidor(maxVentasXsurtidor);
            
        }
    }
    
    
    public void AgregarVentas(int n,int dniNuevo, double m3, String pago ){
        
       this.surtidores[(n-1)].AgregarVentas(dniNuevo, preciom3, m3, pago);
       
    }
    
    public void Verificarm3(double m3Ingresados){
        
        for (int i = 0; i <6; i ++ ){
            
            if(this.surtidores[i] != null){
            
                if (this.surtidores[i].total() < m3Ingresados){

                   this.surtidores[i].cambiarSituacion();

                }
            }
        }
    
    }
    
    public Venta MayorVenta(){
        
        Venta auxventa;
        double maxMonto= -99;
        
        for (int i = 0; i < 6; i ++ ){
            
            if(this.surtidores[i] != null){
        
                if ( this.surtidores[i].BuscarMax() > maxMonto){
                    
                   

                    auxventa = this.MayorVenta();


                }
            }
        }
    
        return auxventa;
    }
    
 
    public String toString(){
    
        String aux;
        
        aux = "ESTACION DE SERVICIO : " +  this.direccion + "\n" + 
                " PRECiO: " +  this.preciom3 + "\n";
        
        
        for (int i = 0 ; i < 6; i ++ ){
            if(this.surtidores[i] != null){
                aux += this.surtidores[i].toString(i);
            }
        }
        
        
       return aux;
    
    }
       
    
    
    
}
