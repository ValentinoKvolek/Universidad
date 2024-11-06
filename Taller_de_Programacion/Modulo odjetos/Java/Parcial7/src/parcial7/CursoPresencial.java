/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial7;

/**
 *
 * @author valen
 */
public class CursoPresencial extends Curso {
    
    private int numAula;  

    public CursoPresencial(int numAula, int anioCursada, int cantAlumnos) {
        super(anioCursada, cantAlumnos);
        this.numAula = numAula;
    }
    
    
    
    public boolean puedeRendir(Alumno alumnoRecibido){
        
        boolean cumple =false;
        
        if (alumnoRecibido.ObtenerAsistencias() >= 3){
            
            cumple= true;
        
        }
                
        return cumple;
    }
}
