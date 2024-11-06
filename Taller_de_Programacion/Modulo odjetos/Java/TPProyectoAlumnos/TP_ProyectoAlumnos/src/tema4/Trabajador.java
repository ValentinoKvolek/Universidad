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
public class Trabajador extends Persona{
      
    private String puesto;

    public Trabajador(String puesto, String unNombre, int unDNI, int unaEdad) {
        super(unNombre, unDNI, unaEdad);
        this.setPuesto(puesto);
    }

    public String getPuesto() {
        return puesto;
    }

    public void setPuesto(String puesto) {
        this.puesto = puesto;
    }
    
    public String toString(){
    
       String aux; 
       aux = super.toString() + 
               " Soy " +  this.getPuesto();
       return aux;
    }
        
}
