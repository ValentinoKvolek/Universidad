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
public class EventoOcasional extends Recital {
    
    private String motivo;
    private String nombreContratante;
    private int dia;
    
    private int diml = 0;

    public EventoOcasional(String motivo, String nombreContratante, int dia, String nombreBanda, int cantidadCanciones) {
        super(nombreBanda, cantidadCanciones);
        this.motivo = motivo;
        this.nombreContratante = nombreContratante;
        this.dia = dia;
    }
    
    public String getMotivo() {
        return motivo;
    }

    public void setMotivo(String motivo) {
        this.motivo = motivo;
    }

    public String getNombreContratante() {
        return nombreContratante;
    }

    public void setNombreContratante(String nombreContratante) {
        this.nombreContratante = nombreContratante;
    }

    public int getDia() {
        return dia;
    }

    public void setDia(int dia) {
        this.dia = dia;
    }
    
    public void agregarTema(String temaAgregado){
        if (diml < super.getCantCanciones()){
            super.getListaCanciones()[diml] = temaAgregado;
            diml ++;
        }
    }
    
    public String actuar(){
        
        String aux;
        
        aux = "un momento...";
        
        if( getMotivo().equals("beneficiencia")){
            return aux = " recuerden colaborar con : " + getNombreContratante() + super.actuar();
        }
        else if (getMotivo().equals("TV")){
            return aux = "Saludos amigos televidentes" + super.actuar();
        }
        else if(getMotivo().equals("Privado")){
            return aux = "Un feliz cumpleaños para:" + getNombreContratante() + super.actuar();
        }
        return aux;
    }
    
    public double calcularCosto(){
        double total=0;
        
       if( getMotivo().equals("beneficiencia")){
           total =0;
       }
       else if (getMotivo().equals("TV")){
            total = 50000;
       }
       else if(getMotivo().equals("Privado")){
           
           total = 150000;
       }
        return total;
    }
    
}
