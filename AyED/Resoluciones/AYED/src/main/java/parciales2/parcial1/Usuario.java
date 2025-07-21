package main.java.parciales2.parcial1;

public class Usuario {

    String nombreUsuario;
    int distancia;

    @Override
    public String toString() {
        return
                 nombreUsuario + ' ' +
                 + distancia ;
    }

    public Usuario(String nombreUsuario, int distancia) {
        this.nombreUsuario = nombreUsuario;
        this.distancia = distancia;
    }

    public String getNombreUsuario() {
        return nombreUsuario;
    }

    public void setNombreUsuario(String nombreUsuario) {
        this.nombreUsuario = nombreUsuario;
    }

    public int getDistancia() {
        return distancia;
    }

    public void setDistancia(int distancia) {
        this.distancia = distancia;
    }
}
