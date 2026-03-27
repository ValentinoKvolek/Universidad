package org.example;

import java.util.Date;

public class Mamifero {

    private String id;
    private String especie;
    private Date fechaNacimiento;
    private Mamifero padre;
    private Mamifero madre;

    public Mamifero(String id) {
        this.id = id;
    }
    public Mamifero(){

    }

    public Mamifero getPadre() {

        return padre;
    }

    public void setPadre(Mamifero padre) {
        this.padre = padre;
    }

    public Mamifero getMadre() {
        return madre;
    }

    public void setMadre(Mamifero madre) {
        this.madre = madre;
    }

    public void setIdentificador(String identificador){
        this.id = identificador;
    }
    public String getIdentificador(){
        return this.id;
    }
    public void setEspecie( String especieInput){
        this.especie = especieInput;
    }
    public String getEspecie(){
        return this.especie;
    }
    public void setFechaNacimiento(Date fecha){
        this.fechaNacimiento = fecha;
    }
    public Date getFechaNacimiento(){
        return this.fechaNacimiento;
    }
    public Mamifero getAbueloMaterno(){

        if( this.madre != null && this.madre.getPadre() != null){
            return this.madre.getPadre();
        }
        return null;

    }
    public Mamifero getAbueloPaterno(){

        if(this.padre != null &&this.padre.getPadre() != null){

            return  this.padre.getPadre();

        }
        return null;
    }

    public Mamifero getAbuelaMaterna(){

        if( this.madre != null && this.madre.getMadre() != null){
            return this.madre.getMadre();
        }

        return null;
    }

    public Mamifero getAbuelaPaterna(){
        if(this.padre != null && this.padre.getMadre() != null){
            return  this.padre.getMadre();
        }
        return null;
    }

    public boolean tieneComoAncestroA(Mamifero unMami) {

        if (unMami == null || this.equals(unMami)) {
            return false;
        }

        if (padre != null && padre.equals(unMami)) return true;
        if (madre != null && madre.equals(unMami)) return true;


        boolean ancestroPorPadre = (padre != null && padre.tieneComoAncestroA(unMami));
        boolean ancestroPorMadre = (madre != null && madre.tieneComoAncestroA(unMami));

        return ancestroPorPadre || ancestroPorMadre;




    }


}
