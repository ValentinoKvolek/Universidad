/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial1;

/**
 *
 * @author valen
 */
public class Empresa {
    
    private String nombre;
    private String direccion;
    private Director directorEjecutivo;
    private int nSucursales;
    private Encargado [] vectorEncargados;
    

    public Empresa(String nombre, String direccion, Director directorEjecutivo, int numeroSucursales) {
        this.nombre = nombre;
        this.direccion = direccion;
        this.directorEjecutivo = directorEjecutivo;
        this.setnSucursales(numeroSucursales);
        this.setVectorEncargados( new Encargado [ numeroSucursales ] );
    }

    
    
    public int getnSucursales() {
        return nSucursales;
    }

    public void setnSucursales(int nSucursales) {
        this.nSucursales = nSucursales;
    }
    

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public String getDireccion() {
        return direccion;
    }

    public void setDireccion(String direccion) {
        this.direccion = direccion;
    }

    public Director getDirectorEjecutivo() {
        return directorEjecutivo;
    }

    public void setDirectorEjecutivo(Director directorEjecutivo) {
        this.directorEjecutivo = directorEjecutivo;
    }

    public Encargado[] getVectorEncargados() {
        return vectorEncargados;
    }

    public void setVectorEncargados(Encargado[] vectorEncargados) {
        this.vectorEncargados = vectorEncargados;
    }
    
    public void asignarEncargado( Encargado nuevoEncargado, int sucursal){
        
            getVectorEncargados()[sucursal]  = nuevoEncargado;
    }
    
    
    public String toStringAux(){
        
        String aux ;
        aux = " ";
        for (int i = 0 ; i < getnSucursales(); i++){
            
            if(getVectorEncargados()[i] == null){
               
               aux += " Sucursal Libre " ;
            
            }else
            
            aux += getVectorEncargados()[i].toString();
        
        }
        
        return aux;
    
    }
    
   
    public String toString (){
        
        String aux;
            
        aux =  "Nombre : " + this.getNombre() +
                " direccion : " +  this.getDireccion() + 
                    getDirectorEjecutivo().toString() +
                        toStringAux();
       
        
        return aux;
                      
     
    }
    
    
    
}
