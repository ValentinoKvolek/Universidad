package main.java.parciales2.parcial3;

public class Ciudades {

    String nombre;
    int dias;

    public Ciudades( int dias, String nombre) {
        this.nombre = nombre;
        this.dias = dias;
    }

    public int getDias() {
        return dias;
    }

    public void setDias(int dias) {
        this.dias = dias;
    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }
}
