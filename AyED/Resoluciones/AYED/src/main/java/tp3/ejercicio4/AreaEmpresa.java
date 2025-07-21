package main.java.tp3.ejercicio4;

public class AreaEmpresa {

    int latencia;
    String representacion;

    public AreaEmpresa(int latencia, String representacion) {
        this.representacion = representacion;
        this.latencia = latencia;
    }

    public int getLatencia() {
        return latencia;
    }

    public void setLatencia(int latencia) {
        this.latencia = latencia;
    }

    public String getRepresentacion() {
        return representacion;
    }

    public void setRepresentacion(String representacion) {
        this.representacion = representacion;
    }
}
