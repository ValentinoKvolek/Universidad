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
public class Alumno {
    
    private String nombre;
    private int DNI;
    private int asistencia;
    private int cantEvaluacionesAprobadas;

    public Alumno(String nombre, int DNI) {
        this.nombre = nombre;
        this.DNI = DNI;
        this.asistencia = 0;
        this.cantEvaluacionesAprobadas = 0;
    }
    
    
    
    
    public void incrementarAsistencia(){
    
        this.asistencia ++;
    
    }
    
    public void increnmarAutoevaluaciones(){
    
        this.cantEvaluacionesAprobadas ++;
    }
    
    public int ObtenerDni(){
    
        return this.DNI;
    
    }
    
    public int ObtenerAsistencias(){
    
        return this.asistencia;
    }
    
    public int ObtenercantAutoevaluaciones(){
    
        return this.cantEvaluacionesAprobadas;
    }    
    
    
    
}
