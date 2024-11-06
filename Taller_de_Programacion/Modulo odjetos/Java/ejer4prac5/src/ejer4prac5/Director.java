/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package ejer4prac5;

/**
 *
 * @author valen
 */
public class Director {
    
    private String nombreDirector;
    
    private int dni;
    
    private int antiguedad;

    
    public Director(String nombreDirector, int dni, int antiguedad) {
        this.nombreDirector = nombreDirector;
        this.dni = dni;
        this.antiguedad = antiguedad;
    }
    
    
    

    public String getNombreDirector() {
        return nombreDirector;
    }

    public void setNombreDirector(String nombreDirector) {
        this.nombreDirector = nombreDirector;
    }

    public int getDni() {
        return dni;
    }

    public void setDni(int dni) {
        this.dni = dni;
    }

    public int getAntiguedad() {
        return antiguedad;
    }

    public void setAntiguedad(int antiguedad) {
        this.antiguedad = antiguedad;
    }
    
    
    
    
}
