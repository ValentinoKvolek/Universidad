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
public class Corista {
    
    private String nombreCorista;
    
    private int dni;
    
    private int edad;
    
    private int tonoFundamental;
    
    

    public Corista(String nombreCorista, int dni, int edad, int tonoFundamental) {
        this.nombreCorista = nombreCorista;
        this.dni = dni;
        this.edad = edad;
        this.tonoFundamental = tonoFundamental;
    }
    
    
    

    public String getNombreCorista() {
        return nombreCorista;
    }

    public void setNombreCorista(String nombreCorista) {
        this.nombreCorista = nombreCorista;
    }

    public int getDni() {
        return dni;
    }

    public void setDni(int dni) {
        this.dni = dni;
    }

    public int getEdad() {
        return edad;
    }

    public void setEdad(int edad) {
        this.edad = edad;
    }

    public int getTonoFundamental() {
        return tonoFundamental;
    }

    public void setTonoFundamental(int tonoFundamental) {
        this.tonoFundamental = tonoFundamental;
    }
    
    public String toString(){
        String aux;
            aux = " nombre: " + getNombreCorista() + 
                    " dni " + getDni() + 
                        " edad " +  getEdad()+
                            " tono " + getTonoFundamental();
        return aux;
    }
    
    
}
