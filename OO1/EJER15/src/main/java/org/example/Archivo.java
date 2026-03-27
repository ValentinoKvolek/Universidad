package org.example;

public class Archivo {
    private  String nombre;

    public Archivo(String s) {
        this.nombre =s;
    }

    public  int tamanio(){
        return nombre.length();
    }
}
