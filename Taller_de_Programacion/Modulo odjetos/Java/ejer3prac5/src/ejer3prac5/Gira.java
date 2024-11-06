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
public class Gira extends Recital {
    
    private String nombreGira;
    private int diml = 0;
    private int cantFechas;
    private int actual=0;
    private Fecha [] vectorFechas;

    public Gira(String nombreGira, Fecha[] vectorFechas, String nombreBanda, int cantidadCanciones, int cantFechas) {
        super(nombreBanda, cantidadCanciones);
        this.nombreGira = nombreGira;
        this.setCantFechas(cantFechas);
        this.setVectorFechas(new Fecha[cantFechas]);
    }

    public int getCantFechas() {
        return cantFechas;
    }

    public void setCantFechas(int cantFechas) {
        this.cantFechas = cantFechas;
    }
    

    public String getNombreGira() {
        return nombreGira;
    }

    public void setNombreGira(String nombreGira) {
        this.nombreGira = nombreGira;
    }

    public Fecha[] getVectorFechas() {
        return vectorFechas;
    }

    public void setVectorFechas(Fecha[] vectorFechas) {
        this.vectorFechas = vectorFechas;
    }
    
    public void agregarTema(String temaAgregado){
        if (diml < super.getCantCanciones()){
            super.getListaCanciones()[diml] = temaAgregado;
            diml ++;
        }
    }
    
    public void agregarFecha(Fecha nuevaFecha){
        if (diml < getCantFechas()){
            getVectorFechas()[diml] =  nuevaFecha;
            diml ++;
        }
    }
    
    public String actuar(){
    
     String aux =  "Buenas noches " + getVectorFechas()[actual].getCiudad() +
                        super.actuar();
                    actual ++;
     
     return aux;
    
    }
    
    
    public double calcularCosto(){
        double total=0;
        
        for (int i= 0; i < diml; i++ ){
            total += 30000;
        }
        return total;
    }
}
