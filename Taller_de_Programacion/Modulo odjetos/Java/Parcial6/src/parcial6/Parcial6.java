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
public class Parcial6 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        
        Estacion e;
        
        e = new Estacion("diagonal 73",100,3);
        
        
        Venta v;
        
        e.AgregarVentas(1, 4284, 40, "debito");
        e.AgregarVentas(2, 424, 50, "efectivo");
        e.AgregarVentas(4, 4284, 20, "debito");
        e.AgregarVentas(4, 4285234, 20, "debito");
        e.AgregarVentas(4,  1384, 300, "debito");
        e.AgregarVentas(6, 1384, 03, "debito");
  
           
        
        e.Verificarm3(90);
        
        System.out.println("EL MAYOR MONTO FUE: " + e.MayorVenta());
        
        System.out.println(e);
    }
    
}
