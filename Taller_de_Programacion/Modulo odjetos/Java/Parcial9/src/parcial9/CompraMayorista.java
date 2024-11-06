/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial9;

/**
 *
 * @author valen
 */
public class CompraMayorista extends Compra {
    
    private int CUIT;

    public CompraMayorista(int CUIT, int nroCompra, int cantMaxComprados) {
        super(nroCompra, cantMaxComprados);
        this.CUIT = CUIT;
    }
    
    
    public String toString(){
    
        String aux;
        
        aux = super.toString() + " CUIT  : " +  this.CUIT;
        
        return aux;
        
    }
}
