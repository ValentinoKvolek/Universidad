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
public abstract class Alumno {
    
    private int DNI;
    private String nombre;
    private int materiasAprobadas= 0;
    private int maxM;
    private int diml =0;
    private Materia [] vector;

    public Alumno(int DNI, String nombre, int MaxMaterias) {
        this.DNI = DNI;
        this.nombre = nombre;
        this.maxM = MaxMaterias;
        this.vector= new Materia [MaxMaterias];
    }

    public void agregarMateriaAprobada(Materia nuevaMateria){
        
           if(nuevaMateria.aprobada() == true){
            
            this.vector[diml] = nuevaMateria;
            diml++;
       
        }
    }
    
    
   public boolean determinarGradudado(){
       
        boolean cumple = false;
        boolean tesis = false;
        
        int cant =0;
        
       for (int i = 0; i < diml;  i++ ){
           
           if(this.vector[i].tesisAprobada()== true){
               tesis = true;
           }
  
           cant ++;
       }
       
       if( cant >= this.materiasAprobadas && tesis == true){
           cumple = true;
       }
       
       return cumple;
   }
    
   public String toString(){
   
    String aux;
        aux = " DNI : " + this.DNI + "n" + 
                " NOMBRE: " + this.nombre + "n" + 
                    "MATERIAS APROBADAS : ";
        
                for (int i =0; i < diml;  i++){
                   aux += this.vector[i].toString() + "\n";
                }
        aux += " esta graduado: ? " + this.determinarGradudado();
        
        return aux;
        
   }
    
}
