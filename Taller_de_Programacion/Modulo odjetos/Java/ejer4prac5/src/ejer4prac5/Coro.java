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
public abstract class Coro {
    
    private String nombreCoro;
    
    private Director director;
    
    private Corista corista;
    

    public Coro(String nombreCoro, Director director, Corista corista) {
        this.nombreCoro = nombreCoro;
        this.director = director;
        this.corista = corista;
    }
    
    

    public String getNombreCoro() {
        return nombreCoro;
    }

    public void setNombreCoro(String nombreCoro) {
        this.nombreCoro = nombreCoro;
    }

    public Director getDirector() {
        return director;
    }

    public void setDirector(Director director) {
        this.director = director;
    }

    public Corista getCorista() {
        return corista;
    }

    public void setCorista(Corista corista) {
        this.corista = corista;
    }
    
    public abstract void agregarCorista(Corista corieta);
    
    public abstract boolean lleno();
    
    public abstract boolean ordenado();
    
    public String toString(){
        
        String aux;
        
        aux = "nombre del coro : " + getNombreCoro() + 
                " nombre director : " + getDirector().getNombreDirector() +
                    " dni del director : " +  getDirector().getDni() + 
                        " antiguedad : " +  getDirector().getAntiguedad();
        return aux;
                        
    }
}
