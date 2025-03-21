package tp1.ejercicio3;

public class Profesor {
    String nombre;
    String apellido;
    int comision;
    String email;
    String catedra;
    String facultad;

    public Profesor(String nombre, String apellido, int comision, String email, String catedra, String facultad) {
        this.nombre = nombre;
        this.apellido = apellido;
        this.comision = comision;
        this.email = email;
        this.catedra = catedra;
        this.facultad = facultad;
    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public String getApellido() {
        return apellido;
    }

    public void setApellido(String apellido) {
        this.apellido = apellido;
    }

    public int getComision() {
        return comision;
    }

    public void setComision(int comision) {
        this.comision = comision;
    }

    public String getEmail() {
        return email;
    }

    public void setEmail(String email) {
        this.email = email;
    }

    public String getCatedra() {
        return catedra;
    }

    public void setCatedra(String catedra) {
        this.catedra = catedra;
    }

    public String getFacultad() {
        return facultad;
    }

    public void setFacultad(String facultad) {
        this.facultad = facultad;
    }

    public String tuDatos() {
        return "Profesor{" +
                "nombre='" + getNombre() + '\'' +
                ", apellido='" + getApellido() + '\'' +
                ", comision=" + getComision() + '\''+
                ", email='" + getEmail() + '\'' +
                ", catedra='" + getCatedra() + '\'' +
                ", facultad='" + getFacultad() + '\'' +
                '}';
    }
}
