/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema3;
import PaqueteLectura.GeneradorAleatorio;
/**
 *
 * @author valen
 */
public class Habitacion {
    
    
    private double Costo; ;
    private boolean Situacion;
    private Cliente nuevocliente;
    
     public Habitacion() {
        this.Costo = GeneradorAleatorio.generarDouble(6001)+ 2000;
        this.Situacion = false;
        this.nuevocliente =null;
        
    }

 
    public double getCosto() {
        return Costo;
    }

    public void setCosto(double Costo) {
        this.Costo = Costo;
    }

    public boolean isSituacion() {
        return Situacion;
    }

    public void setSituacion(boolean Situacion) {
        this.Situacion = Situacion;
    }

    public Cliente getNuevocliente() {
        return nuevocliente;
    }

    public void setNuevocliente(Cliente nuevocliente) {
        this.nuevocliente = nuevocliente;
    }
    
    
    
    
    
    
}
