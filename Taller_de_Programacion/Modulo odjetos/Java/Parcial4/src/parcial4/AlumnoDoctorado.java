/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial4;

/**
 *
 * @author valen
 */
public class AlumnoDoctorado extends Alumno{
     
    private String titulo;
    private String universidad;
    private int diml = 0;

    public AlumnoDoctorado(String titulo, String universidad, int DNI, String nombre,  int MaxMaterias) {
        super(DNI, nombre, MaxMaterias);
        this.titulo = titulo;
        this.universidad = universidad;
    }
    
    
     
    public String toString(){
        
        String aux;
        
        aux = super.toString() + "\n" +
                  "TITULO: " +  this.titulo + "\n" +
                    " universidad: " + this.universidad;
                  
        return aux;
    }
    
    
}
