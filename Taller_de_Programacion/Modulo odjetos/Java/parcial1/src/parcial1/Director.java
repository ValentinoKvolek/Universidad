/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial1;

/**
 *
 * @author valen
 */
public class Director extends Empleado{
    
    private double montoViaticos;

    public Director(double montoViaticos, String nombre, int dni, int anioIngreso, double sueldoBasico) {
        super(nombre, dni, anioIngreso, sueldoBasico);
        this.montoViaticos = montoViaticos;
    }
    
    
    
    

    public double getMontoViaticos() {
        return montoViaticos;
    }

    public void setMontoViaticos(double montoViaticos) {
        this.montoViaticos = montoViaticos;
    }
    
    
    public double sueldoACobrar(){
    
        double total = super.getSueldoBasico();
        
        if( (2024 - super.getAnioIngreso()) > 20 ){
            total += (super.getSueldoBasico() * 0.10);
        }
        
        total += this.getMontoViaticos();
        
        return total;
    
    }
    
}
