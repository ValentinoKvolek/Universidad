/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial2;

/**
 *
 * @author valen
 */
public class Torneo {
    private String nombre;
    private int cantFechas;
    
    private int cantGoleadores;
    
    private Goleador [][] tablaGoleadores;
    
    private int [] vectorCantGoleadores;

    public Torneo(String nombre,int cantFechas,int cantGoleadores) {
        this.nombre = nombre;
        this.cantFechas = cantFechas;
        this.cantGoleadores = cantGoleadores;
        this.setTablaGoleadores(new Goleador [cantFechas][cantGoleadores]);
        this.setVectorCantGoleadores(new int [cantFechas]);
    }

    public int[] getVectorCantGoleadores() {
        return vectorCantGoleadores;
    }

    public void setVectorCantGoleadores(int[] vectorCantGoleadores) {
        this.vectorCantGoleadores = vectorCantGoleadores;
    }

    public Goleador[][] getTablaGoleadores() {
        return tablaGoleadores;
    }

    public void setTablaGoleadores(Goleador[][] tablaGoleadores) {
        this.tablaGoleadores = tablaGoleadores;
    }
    
    public void inicializar(){
        
        for (int i = 0; i < this.cantFechas; i++){
            
            this.vectorCantGoleadores[i] = 0;
        
        }
    
    }
    
    public void agregarGoleador(int fechaIngresada, Goleador goleadorNuevo){
    
           this.tablaGoleadores[fechaIngresada][this.vectorCantGoleadores[fechaIngresada]]  = goleadorNuevo;
           this.vectorCantGoleadores[fechaIngresada]++;
    }
       
    public Goleador goleadorConMenosGoles( int fecha){
        
        Goleador menor;
        menor = new Goleador(" "," ",0);
        int min = Integer.MAX_VALUE;
        
            for (int j = 0; j < this.vectorCantGoleadores[fecha]; j++){
                 
                if ( this.tablaGoleadores[fecha][j].getCantGoles() < min){
                    min = this.tablaGoleadores[fecha][j].getCantGoles();
                    menor = this.tablaGoleadores[fecha][j];
   
                }
            }  
        return menor;
    }
    
    public int cantGoleadoresTotal(){
        
        int total =0;
        for (int i = 0 ; i < this.cantFechas ; i++ ){
            
            total += this.vectorCantGoleadores[i];
            
            
        }
        return total;
    }
    
    public String toString(){
        
            String aux = "TORNEO 2024: " + this.nombre + "\n"; 
            
            for (int i = 0;  i < this.cantFechas; i++){
                    
                aux += "La cantidad de goleadores en la fecha " + (i + 1) + " fueron: " + this.vectorCantGoleadores[i] + "\n";
            
                for (int j = 0; j < this.vectorCantGoleadores[i] ; j++){
                    
                     aux += "Goleador: " + this.tablaGoleadores[i][j].toString() + "\n";
                }
            
            }
                
        return aux;
    
    }
        
        
        
        
        
    
    
    
    
}
