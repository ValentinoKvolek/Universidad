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
public class Parcial7 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        
        CursoPresencial c1;
        CursoDistancia c2;
        
        c1 =  new CursoPresencial(3,5,10);
        
        c2 =  new CursoDistancia("www.yoni.com",5,10);
        
        
        Alumno nuevoAlumno;
        nuevoAlumno = new Alumno("valen",4283);
               
        Alumno nuevoAlumno2;
        nuevoAlumno2 = new Alumno("yoni",4212383);
          
        Alumno nuevoAlumno3;
        nuevoAlumno3 = new Alumno("juanpi",42483);
        
                
        Alumno nuevoAlumno4;
        nuevoAlumno4 = new Alumno("fermin",42833);
                
        Alumno nuevoAlumno5;
        nuevoAlumno5 = new Alumno("theo",42483);
        
        
        
        System.out.println(c1.agregarAlumnos(nuevoAlumno));
       System.out.println(c2.agregarAlumnos(nuevoAlumno2));
        System.out.println(c1.agregarAlumnos(nuevoAlumno3));
       System.out.println(c2.agregarAlumnos(nuevoAlumno4));
        System.out.println(c1.agregarAlumnos(nuevoAlumno5));
        
        
        c1.IncrementarAsistencia(4283);
        c1.IncrementarAsistencia(4283);
        c1.IncrementarAsistencia(4283);
        
        c2.IncrementarAsistencia(42833);
        c2.AprobarAutoevaluacion(42833);
        c2.AprobarAutoevaluacion(42833);
        c2.AprobarAutoevaluacion(42833);
        
        c2.IncrementarAsistencia(4212383);
        
        
        System.out.println("la cantidad de alumnos del curso precensial que pueden rendir son: " +c1.cantPuedenRendir()+"\n");
        
        System.out.println("la cantidad de alumnos del curso online que pueden rendir son: " + c2.cantPuedenRendir()+"\n");
        
        
        
        
    }
    
}
