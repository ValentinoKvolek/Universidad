/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema3;

/**
 *
 * @author valen
 */
public class Autor {
    
    private String nombreAutor;
    private String Biografia;
    private String Origen;

    public Autor(String nombreAutor, String Biografia, String Origen) {
        this.nombreAutor = nombreAutor;
        this.Biografia = Biografia;
        this.Origen = Origen;
    }

    public String getNombreAutor() {
        return nombreAutor;
    }

    public void setNombreAutor(String nombreAutor) {
        this.nombreAutor = nombreAutor;
    }

    public String getBiografia() {
        return Biografia;
    }

    public void setBiografia(String Biografia) {
        this.Biografia = Biografia;
    }

    public String getOrigen() {
        return Origen;
    }

    public void setOrigen(String Origen) {
        this.Origen = Origen;
    }

    @Override
    public String toString() {
        return "Autor{" + "nombreAutor=" + nombreAutor + ", Biografia=" + Biografia + ", Origen=" + Origen + '}';
    }
    
}
