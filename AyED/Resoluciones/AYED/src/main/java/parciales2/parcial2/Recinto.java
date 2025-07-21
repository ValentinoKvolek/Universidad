package main.java.parciales2.parcial2;

public class Recinto {

    String nombre;
    int tiempo;

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public int getTiempo() {
        return tiempo;
    }

    public void setTiempo(int tiempo) {
        this.tiempo = tiempo;
    }


    public Recinto(String nombre, int tiempo) {
        this.nombre = nombre;
        this.tiempo = tiempo;
    }

    public Recinto() {

    }


    @Override
    public String toString() {
        return "Recinto{" +
                "nombre='" + nombre + '\'' +
                ", tiempo=" + tiempo +
                '}';
    }
}
