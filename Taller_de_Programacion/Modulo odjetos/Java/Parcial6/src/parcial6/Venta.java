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
public class Venta {
    
    private int DNI;
    private double cantM3cargados;
    private double montoAbonado;
    private String metodoDePago;

    public Venta(int DNI, double precioDelaestacion, double cantLitros, String metodoDePago) {
        
        
        this.DNI = DNI;
        this.cantM3cargados = cantM3cargados;
        this.montoAbonado = (precioDelaestacion * cantLitros);
        this.metodoDePago = metodoDePago;
        
        
    }
    
   public double obternerm3(){
   
       return cantM3cargados;   
       
   }
   
      public double obternerMonto(){
   
       return this.montoAbonado;   
       
   }
    
      
  public String toString(){
  
      String aux;
      
      aux = "DNI: " +  this.DNI + "\n" +
               "cantM3 " + this.cantM3cargados + "\n" + 
                    "monto Abonado " +  this.montoAbonado + "\n";
      return aux;
  
  
  }
}
