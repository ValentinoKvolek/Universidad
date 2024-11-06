/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial8;

/**
 *
 * @author valen
 */
public class Paciente {
    
    private String nombre;
    private boolean tieneObraSocial;
    private double costo;
    
    
    public Paciente(String nombre, boolean tieneObraSocial, double costo) {
        this.nombre = nombre;
        this.tieneObraSocial = tieneObraSocial;
        this.costo = costo;
    }
    
    
    public String informarPaciente(){
    
        String aux;
        
        aux = "Nombre del paciente que se va a eliminar el turno es : " +  this.nombre;
        
        return aux;
    }
    
    public double obtenerCosto(){
    
    
        return this.costo;
    }
    
    
    
    
}
