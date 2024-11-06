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
public class Materia {
    private String nombreMateria;
    private double nota;
    private  String fecha;

    public Materia(String nombreMateria, double nota, String fecha) {
        this.nombreMateria = nombreMateria;
        this.nota = nota;
        this.fecha = fecha;
    }
    
    
    public boolean aprobada(){
        
        boolean cumple = false;
        if(this.nota > 4){
            cumple= true;
        }
    
        return cumple;
    }
    
    
    public boolean tesisAprobada(){
        
        boolean aux = false;
        
        if(this.nombreMateria.equals("TESIS")){
            aux = true;
        }
            
        return aux;
           
    }
    
    public String toString(){
    
        String aux;
        
        aux = "NOMBRE : " + this.nombreMateria +  "\n " +
                "NOTA : " +  this.nota + "\n" + 
                    "fecha : " +  this.fecha ;
        return aux;
    }
        
        
}
