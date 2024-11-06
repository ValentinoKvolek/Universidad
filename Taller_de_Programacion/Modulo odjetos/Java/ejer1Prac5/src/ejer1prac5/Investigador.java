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
public class Investigador {
    private String nombre;
    private int categoria;
    private String especialidad;
    private int diml = 0;
    private Subsidio [] vectorSubsidios;

    public Investigador(String nombre, int categoria, String especialidad) {
        setNombre(nombre);
        setCategoria(categoria);
        setEspecialidad(especialidad);
        setVectorSubsidios(new Subsidio[5]);
    }
    

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public int getCategoria() {
        return categoria;
    }

    public void setCategoria(int categoria) {
        this.categoria = categoria;
    }

    public String getEspecialidad() {
        return especialidad;
    }

    public void setEspecialidad(String especialidad) {
        this.especialidad = especialidad;
    }

    public Subsidio[] getVectorSubsidios() {
        return vectorSubsidios;
    }

    public void setVectorSubsidios(Subsidio[] vectorSubsidios) {
        this.vectorSubsidios = vectorSubsidios;
    }
    
    public void agregarSubsidio(Subsidio unSubsidio){
        if (diml + 1 < 5) {
        this.vectorSubsidios[diml] = unSubsidio;
            diml ++;
        }
        else System.out.println("vector lleno!");
    }
    
    public double sumaSubcidios(){
        double total =0;
        for (int i = 0 ; i < diml; i++){
           total += this.vectorSubsidios[i].getMonto();
        }
        return total;
    }
    
    public void setearTrue(){
        for (int i =0 ; i <diml; i++){
            this.vectorSubsidios[i].setOtorgado(true);
        }
    }
    
    
    public String toString(){
       String aux;
            aux = " nombre Investigador : " +  getNombre()+
                       " categoria : " + getCategoria() +
                            " especialidad : " + getEspecialidad() +
                               " monto total de subsidios: " + sumaSubcidios();
            return aux;
       
    }
    
}
