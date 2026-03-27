package org.example;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;


public class Carpeta {
    private String nombre;
    private List<Email> emails = new ArrayList<>();



    public Carpeta(String nombre) {
        this.nombre = nombre;
    }

    public String getNombre(){
        return this.nombre;
    }

    public boolean agregarEmail(Email email){
        this.emails.add(email);
        return true;
    }


    public void mover(Email email, Carpeta destino) {
        if(emails.contains(email)){
            destino.agregarEmail(email);
            this.emails.remove(email);
        }
    }

    public Email buscarEmail( String texto){
        for (Email email : emails){
          if(email.contiene(texto)) return email;
        }
        return null;
    }

    public int espacioOcupado(){
        int suma = 0;
        for (Email email : emails){
            suma += email.espacioOcupado();
        }
        return suma;
    }

    public int cantidadMails(){
        return emails.size();
    }

    public Map<String,Integer> cantMailCategoria(){
        Map<String, Integer> categorias = new HashMap<>();

        for (Email email : emails) {
            String categoria;
            int tam = email.espacioOcupado();

            if (tam <= 300) categoria = "Pequeño";
            else if (tam <= 500) categoria = "Mediano";
            else categoria = "Grande";

            categorias.put(categoria, categorias.getOrDefault(categoria, 0) + 1);
        }

        return categorias;

    }




}
