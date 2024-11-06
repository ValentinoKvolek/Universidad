/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package ejer3prac5;

/**
 *
 * @author valen
 */
public abstract class Recital {
    
    private String nombreBanda;
    
    private String [] listaCanciones;
    
    private int pos = 0;
    
    private int cantCanciones;

    public Recital(String nombreBanda, int cantidadCanciones) {
        this.nombreBanda = nombreBanda;
        this.setCantCanciones(cantCanciones);
        this.setListaCanciones(new String [cantidadCanciones]);
    }   

    public int getCantCanciones() {
        return cantCanciones;
    }

    public void setCantCanciones(int cantCanciones) {
        this.cantCanciones = cantCanciones;
    }

    public String getNombreBanda() {
        return nombreBanda;
    }

    public void setNombreBanda(String nombreBanda) {
        this.nombreBanda = nombreBanda;
    }

    public String[] getListaCanciones() {
        return listaCanciones;
    }

    public void setListaCanciones(String[] listaCanciones) {
        this.listaCanciones = listaCanciones;
    }
   
    
    public abstract void agregarTema(String tema);
    
public String actuar() {
    String aux;
    
    if (pos < getCantCanciones() && getListaCanciones()[pos] != null) {
        aux = "Y ahora tocaremos: " + getListaCanciones()[pos];
        pos++;
    } else {
        aux = "No hay más canciones para tocar.";
    }

    return aux;
}

    public abstract double calcularCosto();
}
