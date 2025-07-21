package main.java.parciales2.parcial6;

public class Ciudad {
    String nombre;
    int limiteDias;

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public int getLimiteDias() {
        return limiteDias;
    }

    public void setLimiteDias(int limiteDias) {
        this.limiteDias = limiteDias;
    }

    public Ciudad(String nombre, int limiteDias) {
        this.nombre = nombre;
        this.limiteDias = limiteDias;
    }

    @Override
    public String toString() {
        return "Ciudad{" +
                "nombre='" + nombre + '\'' +
                ", limiteDias=" + limiteDias +
                '}';
    }
}
