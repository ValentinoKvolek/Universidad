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
public class Encargado extends Empleado{
    
    private int cantEmpladosACargo;

    public Encargado(int cantEmpladosACargo, String nombre, int dni, int anioIngreso, double sueldoBasico) {
        super(nombre, dni, anioIngreso, sueldoBasico);
        this.cantEmpladosACargo = cantEmpladosACargo;
    }
   
    

    public int getCantEmpladosACargo() {
        return cantEmpladosACargo;
    }

    public void setCantEmpladosACargo(int cantEmpladosACargo) {
        this.cantEmpladosACargo = cantEmpladosACargo;
    }
    
    public double sueldoACobrar(){
    
        double total = super.getSueldoBasico();
        
        if( (2024 - super.getAnioIngreso()) > 20 ){
            total += (super.getSueldoBasico() * 0.10);
        }
        
        total += (1000*this.getCantEmpladosACargo());
        
        return total;
    
    }
    
}
