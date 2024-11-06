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
public class Chileras extends Coro {
    
    
    private int hilera =0;
    private int columna =0;
  
    private int dimension;
    
    private Corista [][] coristas;

    public Chileras(String nombreCoro, Director director, Corista corista, int dimension) {
        super(nombreCoro, director, corista);
        this.setDimension(dimension);
        this.setCoristas(new Corista [dimension][dimension]);
    }

    public int getDimension() {
        return dimension;
    }

    public void setDimension(int dimension) {
        this.dimension = dimension;
    }

    

    public Corista[][] getCoristas() {
        return coristas;
    }

    public void setCoristas(Corista[][] coristas) {
        this.coristas = coristas;
    }
    
    public void agregarCorista(Corista nuevoCorista){
        
        if(hilera < getDimension()){
            
            if(columna< getDimension()){
                
                getCoristas()[hilera][columna] = nuevoCorista;
                columna++;
                
            }
            else { // si la hilera esta llena entonces reseteo
                    columna =0;
                    hilera ++;
            }
        }      
    }
    
    public boolean lleno(){
           
        
        boolean aux = false;
        
        if( (hilera +1) == getDimension()){
            if( (columna +1) == getDimension()){
                aux = true;
            }
        }
    
        return aux;
    }
    

    public boolean ordenado() {

       int i,j,max;
       j = 0;
       i =0;
       max= 999999;
       boolean cumple  = true;
       int tonoActual = getCoristas()[i][0].getTonoFundamental();
       
       while ( cumple == true && i < (getDimension() - 1)){

           while(cumple == true && j < (getDimension() - 1)){
                if(tonoActual != getCoristas()[i][j].getTonoFundamental() ){
                    cumple = false;
                }
                j ++;
            }
           j = 0;
           i++;
           tonoActual = getCoristas()[i][0].getTonoFundamental();  
       }
       i = 0;
       j= 0;
       
       while (cumple == true && i < (getDimension() - 1)){
           if(max <  getCoristas()[i][0].getTonoFundamental()){
               cumple=  false;
           }
           max= getCoristas()[i][0].getTonoFundamental();
           i++;
       }
       
   
       
       return cumple;
       
       
       
       
     }




    
    public void informar(){
        
        System.out.println(super.toString());
        
        for(int i =0; i< hilera ; i++){
            
            for (int j=0; j < getDimension(); j++){
                System.out.println(getCoristas()[i][j].toString());
            }
            
        }
    
    }
}
