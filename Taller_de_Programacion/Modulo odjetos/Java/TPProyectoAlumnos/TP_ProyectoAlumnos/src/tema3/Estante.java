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
public class Estante {
    
    private int  cantLibrosAlmacenados =0;
    
    private int maxLibros =20;
    
    
    private Libro [] vectorLibros = new Libro [maxLibros];

    public Estante() {}
    
    public void agregarLibro(Libro nuevoLibro){
        
           if (cantLibrosAlmacenados >= maxLibros) return;
           
           vectorLibros[cantLibrosAlmacenados] = nuevoLibro;
           
           cantLibrosAlmacenados++;
    }
    
    public int getCantLibros(){
          return cantLibrosAlmacenados;
    }
    
     public boolean estaLleno() {
        return cantLibrosAlmacenados >= maxLibros;
    }
     
    public Libro obtenerLibro (String nombre){
        
           for (int i = 0;  i<cantLibrosAlmacenados; i++){
               if(vectorLibros[i].getTitulo().equals(nombre)){
                    return vectorLibros[i];
               }
           }
           return null;
    }
    
    public Estante(int maxLibros) {    //resivo un valor maximo que me da el usuario
        this.maxLibros = maxLibros;
        this.vectorLibros= new Libro [maxLibros];
    }
  
    
}
