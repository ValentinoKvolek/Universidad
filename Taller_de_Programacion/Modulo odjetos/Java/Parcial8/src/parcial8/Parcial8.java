/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial8;

/**
 *
 * @author valen
 */
public class Parcial8 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        
        Sistema nuevoSistema;
        nuevoSistema = new Sistema ();
        
        
        Paciente p;
        
        p = new Paciente("fermin", true, 400);
        
         Paciente p2;
        
        p2 = new Paciente("juanpi", false , 100);
        
         Paciente p3;
        
        p3 = new Paciente("yoni", false, 900);
         Paciente p4;
        
        p4 = new Paciente("valen", true, 200);
        
        
        
        
        nuevoSistema.agendarPaciente(p, 1, 1);
        nuevoSistema.agendarPaciente(p2, 5, 6);
        nuevoSistema.agendarPaciente(p3, 2, 1);
        nuevoSistema.agendarPaciente(p4, 1, 2);
        
        System.out.println(nuevoSistema.liberarAgenda(5));
        
        
        System.out.println("el costo semanal es de : " + nuevoSistema.calcularCostoSemanal());
        
        
    }
    
}
