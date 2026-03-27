package org.example;

import java.util.ArrayList;
import java.util.List;

public class ClienteDeCorreo {

    private Carpeta inbox;
    private List<Carpeta> carpetas;

    public ClienteDeCorreo() {
        this.carpetas = new ArrayList<>();
        this.inbox = new Carpeta("Inbox");
        this.carpetas.add(inbox);
    }

    public void recibir(Email email){
        if(!this.inbox.agregarEmail(email)){
            System.out.printf("La swap de la ram se lleno y no se puedo agregar a la estructura.");
        }
    }

    public Email buscar(String texto) {
        for (Carpeta carpeta : carpetas) {
            Email encontrado = carpeta.buscarEmail(texto);
            if (encontrado != null) return encontrado;
        }
        return null;
    }

    public int espacioOcupado(){
        int suma = 0;
        for (Carpeta carpeta : carpetas){
            suma += carpeta.espacioOcupado();
        }
        return suma;
    }

    public int cantidadTotalMails(){
        int total =0;
        for (Carpeta carpeta:carpetas){
            total+=carpeta.cantidadMails();
        }
        return total;
    }
}
