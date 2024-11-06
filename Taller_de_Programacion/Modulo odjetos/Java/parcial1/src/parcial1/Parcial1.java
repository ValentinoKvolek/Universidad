/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial1;

import PaqueteLectura.GeneradorAleatorio;

/**
 *
 * @author valen
 */
public class Parcial1 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        // TODO code application logic here
        
        GeneradorAleatorio.iniciar();
        
        Empresa nuevaEmpresa;
        
        String nombreEmpresa = GeneradorAleatorio.generarString(5);
        
        String direccionEmpresa = GeneradorAleatorio.generarString(5);
        
        Director nuevoDirector;
        
        nuevoDirector = new Director (100,"FRAN", 43504, 2004, 22204);
        
        
        
        
         
        
        nuevaEmpresa = new Empresa ( nombreEmpresa, direccionEmpresa ,nuevoDirector , 5);
        
        Encargado nuevoEncargado1;
        
        nuevoEncargado1 = new Encargado(10 ,"Valen", 42849894, 2014, 20000);
                
        Encargado nuevoEncargado2;
        
        nuevoEncargado2 = new Encargado(4 ,"luca", 412394, 2019, 22200);
                
        Encargado nuevoEncargado3;
        
        nuevoEncargado3 = new Encargado(1 ,"yoni", 449894, 2021, 2043);
        
        
        nuevaEmpresa.asignarEncargado(nuevoEncargado3, 0);
        nuevaEmpresa.asignarEncargado(nuevoEncargado1, 2);
        nuevaEmpresa.asignarEncargado(nuevoEncargado2, 4);
        
        
        System.out.println(nuevaEmpresa.toString());
                
        
    }
    
}
