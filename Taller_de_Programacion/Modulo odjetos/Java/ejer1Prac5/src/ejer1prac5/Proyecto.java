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
public class Proyecto {
    
    private String nombre;
    private int codigo;
    private String nombreDirector;
    private int diml =0;
    private Investigador [] vectorInvectigadores;

    public Proyecto(String nombre, int codigo, String nombreDirector) {
        
        setNombre(nombre);
        setCodigo(codigo);
        setNombreDirector(nombreDirector);
        setVectorInvectigadores(new Investigador[50]);
        
    }

    
    
    
    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public int getCodigo() {
        return codigo;
    }

    public void setCodigo(int codigo) {
        this.codigo = codigo;
    }

    public String getNombreDirector() {
        return nombreDirector;
    }

    public void setNombreDirector(String nombreDirector) {
        this.nombreDirector = nombreDirector;
    }

    public Investigador[] getVectorInvectigadores() {
        return vectorInvectigadores;
    }

    public void setVectorInvectigadores(Investigador[] vectorInvectigadores) {
        this.vectorInvectigadores = vectorInvectigadores;
    }
    
    
    
    public void agregarInvestigador(Investigador unInvestigador){
        if(diml + 1 < 50){
           this.vectorInvectigadores[diml] = unInvestigador;
           diml++;
        }
        else System.out.println("vector lleno");
    }
    
    
   public double dineroTotalOtorgado(){
       
       double total =0;
       for (int i = 0; i < diml; i++){
           total += this.vectorInvectigadores[i].sumaSubcidios();
       }
       return total;
   }
  
   public void otorgarTodos(String nombre_completo){
       int i =0;
       while (nombre_completo != this.vectorInvectigadores[i].getNombre() && i< diml){
           i++;
       }
       if( nombre_completo == this.vectorInvectigadores[i].getNombre() ) {
           
           this.vectorInvectigadores[i].setearTrue();
       
       }
   }
   public void leerInvestigador(){
       for (int i = 0; i< diml; i++ ){
           System.out.println(this.vectorInvectigadores[i].toString());
       }
          
   }
   
   public void String(){
        System.out.println( "el nombre del proyecto es : " +  getNombre() +
                " el codigo es : " +  getCodigo()+
                    " el nombre del director:" +  getNombreDirector() +
                        " total de dinero otorgado : " + dineroTotalOtorgado() + 
                            " los investigadores son : " );
                               this.leerInvestigador();
                            
  
    } 


}

