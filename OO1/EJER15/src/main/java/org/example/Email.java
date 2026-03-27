package org.example;

import java.util.List;

public class Email {

    private String titulo;
    private String cuerpo;
    private List<Archivo> archivos;

    public Email(String titulo, String cuerpo, List<Archivo> archivos) {
        this.titulo = titulo;
        this.cuerpo = cuerpo;
        this.archivos = archivos;
    }


    public boolean contiene(String texto){
        return  this.titulo.contains(texto) || this.cuerpo.contains(texto);
    }

    public int espacioOcupado(){
        int suma =0;
        for (Archivo archivo : archivos){
            suma += archivo.tamanio();
        }
        return  suma + this.titulo.length() + this.cuerpo.length();
    }


    public String getTitulo(){
        return this.titulo;
    }
    public String getCuerpo(){
        return this.cuerpo;
    }
    public List<Archivo> adjuntos(){
        return  this.archivos;
    }

}
