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
public abstract class Curso {
    
    
    private int anioCursada;
    private int diml = 0;
    private int dimf;
    
    private Alumno [] vector;

    public Curso(int anioCursada, int cantAlumnos) {
        this.anioCursada = anioCursada;
        this.dimf = cantAlumnos;
        this.vector = new Alumno [cantAlumnos];
    }
    
    
    public boolean agregarAlumnos(Alumno nuevoAlumno){
        
       boolean aux = false;
    
       if( diml < dimf ){
           
           System.out.println("AGREGUE");
           this.vector[diml] = nuevoAlumno;
           diml ++;
           aux = true;
       }
       
       return aux;
    }
    
    public void IncrementarAsistencia(int dniAlumno){
        
    
        for (int i = 0 ; i < diml; i ++){
           
            if( this.vector[i] !=  null){
                
                if(this.vector[i].ObtenerDni() == dniAlumno){
                    
                    System.out.println("incremente");
                    
                    this.vector[i].incrementarAsistencia();
                    
                }
            }
            
        }
    }
    
   public void AprobarAutoevaluacion(int dniAlumno){
        
     for (int i = 0 ; i < diml; i ++){
            
            if( this.vector[i] !=  null){
                
                if(this.vector[i].ObtenerDni() == dniAlumno){
            
                    this.vector[i].increnmarAutoevaluaciones();
                    
                }
            }
            
        }
   } 
        
        
  public abstract boolean puedeRendir(Alumno alumnoRecibido);
  
   
  public int cantPuedenRendir(){
      
      int cant = 0;
      
      
      for (int i = 0; i < diml; i++){
          
          if( puedeRendir(this.vector[i]) == true){
          
              cant ++;
          }
  
      }
     
      return cant;
  }
 
    
}
