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
public class CompraMinorista extends Compra {
    
    private boolean jubilado;

    public CompraMinorista(boolean jubilado, int nroCompra, int cantMaxComprados) {
        super(nroCompra, cantMaxComprados);
        this.jubilado = jubilado;
    }
    
    
    public String toString(){
    
        String aux;
        
        aux = super.toString() + " JUBILADO?:  : " +  this.jubilado;
        
        return aux;
        
    }
    
    
    
    
}
