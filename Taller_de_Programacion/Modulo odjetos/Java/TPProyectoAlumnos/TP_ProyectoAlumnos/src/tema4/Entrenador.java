/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema4;

/**
 *
 * @author valen
 */
public class Entrenador extends Empleado{
    
    private int cantCampeonatos;

    public Entrenador(int cantCampeonatos, String nombre, double sueldo, int antiguedad) {
        super(nombre, sueldo, antiguedad);
        this.setCantCampeonatos(cantCampeonatos);
    }
    

    public int getCantCampeonatos() {
        return cantCampeonatos;
    }

    public void setCantCampeonatos(int cantCampeonatos) {
        this.cantCampeonatos = cantCampeonatos;
    }
    
    public double calcularEfectividad(){
        return this.getCantCampeonatos()/super.getAntiguedad();
    }
    
    public  double  calcularSueldoACobrar(){
        
        double sueldoa = this.getSueldo() + (this.getSueldo() * 0.10 * this.getAntiguedad());
        
        if( this.getCantCampeonatos() > 1 && this.getCantCampeonatos() <= 4){
            sueldoa = sueldoa + 5000;
        }
        else if( this.getCantCampeonatos() > 4 &&  this.getCantCampeonatos() <=5) {
            sueldoa = sueldoa + 30000;
        }
        else if(this.getAntiguedad() > 10){
            sueldoa = sueldoa + 50000;
        }
       return sueldoa;
    }
    
    
    public String toString(){
        String aux = super.toString() + 
                        "sueldo a cobrar: " + this.calcularSueldoACobrar() + 
                            " efectividad : " + this.calcularEfectividad();
        return aux;

    }
    
    
    
}
