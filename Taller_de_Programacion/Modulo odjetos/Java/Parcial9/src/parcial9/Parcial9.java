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
public class Parcial9 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        
        CompraMayorista c1;
        CompraMinorista c2;
        
        Producto p1;
        p1 = new Producto(123, 400, "nashe");
        
        Producto p2;
        p2 = new Producto(13, 400, "nashe");
        
        Producto p3;
        p3 = new Producto(12, 3500, "nashe");
        
        Producto p4;
        p4 = new Producto(12412, 200, "nashe");
        
        Producto p5;
        p5 = new Producto(12341, 300, "nashe");
        
        Producto p6;
        p6 = new Producto(125233, 700, "nashe");
        
        
        c1 = new CompraMayorista(424, 40, 4);
        c2 = new CompraMinorista(true, 53, 4);
        
        c1.agregarCompra(p1);
        c1.agregarCompra(p2);
        c1.agregarCompra(p3);
        c2.agregarCompra(p4);
        c2.agregarCompra(p5);
        c2.agregarCompra(p6);
        
        
        System.out.println("TOTAL DE LA COMPRA MAYORISTA : " + c1.totalPrecio()+ "\n");
        System.out.println("TOTAL DE LA COMPRA MINORISTA : " + c2.totalPrecio()+ "\n");
        
        System.out.println(c1.toString());
        System.out.println(c2.toString());

    }
    
}
