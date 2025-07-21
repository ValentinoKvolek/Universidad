package main.java.tp5.ejercicio5;

public class Persona {

    boolean jubilado;
    String nombre;
    String domicilio;

    public Persona(boolean jubilado, String nombre, String domicilio) {
        this.jubilado = jubilado;
        this.nombre = nombre;
        this.domicilio = domicilio;
    }

    public String getDomicilio() {
        return domicilio;
    }

    public void setDomicilio(String domicilio) {
        this.domicilio = domicilio;
    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public boolean isJubilado() {
        return jubilado;
    }

    public void setJubilado(boolean jubilado) {
        this.jubilado = jubilado;
    }
}
