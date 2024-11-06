/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package prac2prac5;

/**
 *
 * @author valen
 */
public class Auto {
    
    private String nombreDueño;
    private int patente;

    public Auto(String nombreDueño, int patente) {
        this.setNombreDueño(nombreDueño);
        this.setPatente(patente);
    }
    

    public String getNombreDueño() {
        return nombreDueño;
    }

    public void setNombreDueño(String nombreDueño) {
        this.nombreDueño = nombreDueño;
    }

    public int getPatente() {
        return patente;
    }

    public void setPatente(int patente) {
        this.patente = patente;
    }
    
   public String toString(){
      
       String aux;
       
       aux = " Nombre dueño : " + getNombreDueño() +
               " Patente : " + getPatente();
       
       return aux;
   }
    
}
