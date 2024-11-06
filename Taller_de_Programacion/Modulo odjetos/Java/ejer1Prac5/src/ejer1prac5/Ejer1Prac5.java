/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package ejer1prac5;

/**
 *
 * @author valen
 */
public class Ejer1Prac5 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        // TODO code application logic here
        
        
        Proyecto nuevoProyecto;
        
        nuevoProyecto = new Proyecto ("MAQUINA DEL MAL", 123, "VALENTINO");
              
        
        Investigador nuevoInvestigador;
        
        nuevoInvestigador = new Investigador("YONI", 2 , "TAPEADOR PROFESIONAL");
        
            Subsidio nuevoSubsidio;

            nuevoSubsidio = new Subsidio(300, "SUBE");

            Subsidio nuevoSubsidio2;

            nuevoSubsidio2 = new Subsidio(400, "FOTOCOPIAS");
            
            nuevoInvestigador.agregarSubsidio(nuevoSubsidio);
            nuevoInvestigador.agregarSubsidio(nuevoSubsidio2);
            

        Investigador nuevoInvestigador2;
        
        nuevoInvestigador2 = new Investigador("JUANPI", 1 , "EXPERTO EN CADP");
        
            nuevoInvestigador2.agregarSubsidio(nuevoSubsidio);
            nuevoInvestigador2.agregarSubsidio(nuevoSubsidio2);
        
        Investigador nuevoInvestigador3;
        
        nuevoInvestigador3 = new Investigador("FERMIN", 4 , "EXPERTO EN TACLADOS");
        
            nuevoInvestigador3.agregarSubsidio(nuevoSubsidio);
            nuevoInvestigador3.agregarSubsidio(nuevoSubsidio2);
            
        
        nuevoProyecto.agregarInvestigador(nuevoInvestigador);
        nuevoProyecto.agregarInvestigador(nuevoInvestigador2);
        nuevoProyecto.agregarInvestigador(nuevoInvestigador3);
       
        
        nuevoProyecto.String();
    }
    
}
