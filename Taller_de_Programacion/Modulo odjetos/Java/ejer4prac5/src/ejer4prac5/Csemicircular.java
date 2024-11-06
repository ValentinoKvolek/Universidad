/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package ejer4prac5;

/**
 *
 * @author valen
 */
public class Csemicircular extends Coro {
    
    private int diml = 0;
    private int dimension;
    private Corista [] coristas;

    public Csemicircular(String nombreCoro, Director director, Corista corista, int dimension) {
        super(nombreCoro, director, corista);
        this.setDimension(dimension);
        this.setCoristas(new Corista [dimension]);
    }

    public int getDimension() {
        return dimension;
    }

    public void setDimension(int dimension) {
        this.dimension = dimension;
    }

    public Corista[] getCoristas() {
        return coristas;
    }

    public void setCoristas(Corista[] coristas) {
        this.coristas = coristas;
    }
    
    
    public void agregarCorista(Corista nuevoCorista){
            
        if (diml < getDimension()){
            
            getCoristas()[diml] = nuevoCorista;
            diml ++;
        }
        
    }
    
    public boolean lleno(){
           
        
        boolean aux = false;
        
        if( (diml +1) == getDimension()){
            
            aux = true;
        }
    
        return aux;
    }
    
    
    public boolean ordenado(){
       
        int max = -1;
        for ( int pos = 0; pos < diml;  pos++ ){
            if (getCoristas()[pos].getTonoFundamental() < max) {
                return false;
            }
            max = getCoristas()[pos].getTonoFundamental();
        }
        return true;
        
        
    }
    
    
    public void informar(){
        
            System.out.println(super.toString());
            
            for (int i = 0; i < diml; i++){
                 System.out.println( getCoristas()[i].toString());
            }
    }
    
}
