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
public class Jugador extends Empleado{
    
    private int PartidosJugados;
    private int CantidadDeGoles;

    public Jugador(int PartidosJugados, int CantidadDeGoles, String nombre, double sueldo, int antiguedad) {
        super(nombre, sueldo, antiguedad);
        this.setPartidosJugados(PartidosJugados);
        this.setCantidadDeGoles(CantidadDeGoles);
    }

    public int getPartidosJugados() {
        return PartidosJugados;
    }

    public void setPartidosJugados(int PartidosJugados) {
        this.PartidosJugados = PartidosJugados;
    }

    public int getCantidadDeGoles() {
        return CantidadDeGoles;
    }

    public void setCantidadDeGoles(int CantidadDeGoles) {
        this.CantidadDeGoles = CantidadDeGoles;
    }
    
    public double calcularEfectividad(){ 
        
        return this.getCantidadDeGoles()/  this.getPartidosJugados();
    }
    
    public double calcularSueldoACobrar(){
        
       double sueldoa = this.getSueldo() + (this.getSueldo() * 0.10 * this.getAntiguedad());
        
        if ( this.calcularEfectividad() > 0.5 ){
            sueldoa = sueldoa+ 268.000;
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
