/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial4;

/**
 *
 * @author valen
 */
public class AlumnoGrado extends Alumno {
    
    private String carrera;
    private int diml =0;

    public AlumnoGrado(String carrera, int DNI, String nombre,  int MaxMaterias) {
        super(DNI, nombre, MaxMaterias);
        this.carrera = carrera;
    }
    
    
 
    public String toString(){
        
        String aux;
        
        aux = super.toString() + "\n" +
                  "CARRERA: " +  this.carrera;
                  
        return aux;
    }
    
}
