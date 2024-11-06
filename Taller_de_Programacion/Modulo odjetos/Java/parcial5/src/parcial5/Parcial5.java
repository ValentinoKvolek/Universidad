/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial5;

import com.sun.xml.internal.bind.v2.runtime.output.SAXOutput;

/**
 *
 * @author valen
 */
public class Parcial5 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        
        SalaPC nuevasala;
        
        nuevasala = new SalaPC(5);
        
        
        Pc pc1;
        
        pc1 = new Pc(200);
        
        Pc pc2;
        
        pc2 = new Pc(100);
        
        
        Pc pc3;
        
        pc3 = new Pc(400);
        
        
        nuevasala.agregarPc(1, pc3);
        
        
        nuevasala.agregarPc(5, pc2);
        
        
        nuevasala.agregarPc(2, pc1);
        
        
        System.out.println("TENDRIAS QUE ESTAR TODAS APAGADAS Y NO INFROMAR NADA" + "\n");
        
        System.out.println(nuevasala.espaciosConPcON());
        
        System.out.println("TENDRIAS QUE ESTAR TODAS APAGADAS Y NO INFROMAR NADA" + "\n");
        
        
        System.out.println("ENCIENDO LA PRIMER PC ");
        
        nuevasala.encederPc();
        
        
        System.out.println("ENCIENDO AL SEGUNDA PC ");
        
        nuevasala.encederPc();
        
        
        System.out.println("TENDRIAS MOSTRAR 2 PC" + "\n");
        
        System.out.println(nuevasala.espaciosConPcON());
        
    }
    
}
