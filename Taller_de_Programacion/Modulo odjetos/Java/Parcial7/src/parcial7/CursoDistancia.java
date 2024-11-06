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
public class CursoDistancia extends Curso{
    
    private String link;
    private int diml =0;
    private int dimf;

    public CursoDistancia(String link, int anioCursada, int cantAlumnos) {
        
        super(anioCursada, cantAlumnos);
        this.dimf = cantAlumnos;
        this.link = link;
   
    }
    
    public boolean puedeRendir(Alumno alumnoRecibido){
        
        boolean cumple = false;
        
        if (alumnoRecibido.ObtenercantAutoevaluaciones()>= 3 && alumnoRecibido.ObtenerAsistencias() >= 1){
            
            cumple= true;
        
        }
                
        return cumple;
    }
    
    
    
    
    
    
}
