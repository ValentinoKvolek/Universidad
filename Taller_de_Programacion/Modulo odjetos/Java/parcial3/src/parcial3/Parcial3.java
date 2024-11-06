/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial3;

/**
 *
 * @author valen
 */
public class Parcial3 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        
        
        Funcion nuevaFuncion;
        
        nuevaFuncion = new Funcion("EL JOROBADO DE NASHE", 4, 12, 5,4);
        
        Butaca nuevaButaca;
        
        nuevaButaca = new Butaca();
         Butaca nuevaButaca2;
        nuevaButaca2 = new Butaca();
         Butaca nuevaButaca3;
        nuevaButaca3 = new Butaca();
         Butaca nuevaButaca4;
        nuevaButaca4 = new Butaca();
        
        
        
        
        System.out.println(nuevaFuncion.ocupar(5, 4,nuevaButaca));
        System.out.println (nuevaFuncion.ocupar(4, 4,nuevaButaca2));
        System.out.println (nuevaFuncion.ocupar(2, 1,nuevaButaca3));
        System.out.println( nuevaFuncion.ocupar(4, 2, nuevaButaca4));
        
 
        
        
        System.out.println("voy a informar todas las butacas de una fila");
        
        System.out.println(nuevaFuncion.todasButacas(4));   
        
        
        System.out.println("las desocupo");
        
        nuevaFuncion.desocupar(1);
        

        
        System.out.println("YA PASE EL SEGUNDO");
        
        
        System.out.println(nuevaFuncion.todasButacas(4));
        
        
        
        System.out.println(nuevaFuncion.toString());
        
        
        
    }
    
}
