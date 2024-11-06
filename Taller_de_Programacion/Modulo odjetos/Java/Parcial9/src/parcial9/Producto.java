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
public class Producto {
    
    private int codigo;
    
    private double precio;
    
    private String descripcion;

    public Producto(int codigo, double precio, String descripcion) {
        this.codigo = codigo;
        this.precio = precio;
        this.descripcion = descripcion;
    }
    
    
    public double obtenerPrecio(){
    
        return this.precio;
    
    }
    
    
    public String toString(){
    
        String aux;
        aux = " CODIGO DEL PRODUCTO: " + this.codigo + 
                " PRECIO: " +  this.precio + 
                    " Descripcion: " +  this.descripcion;
        
        return aux;
    }
    
    
}
