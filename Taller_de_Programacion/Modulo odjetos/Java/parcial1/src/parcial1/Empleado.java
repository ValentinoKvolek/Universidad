/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial1;

/**
 *
 * @author valen
 */
public abstract class Empleado {
    
    private String nombre;
    private int dni;
    private int anioIngreso;
    private double sueldoBasico;

    public Empleado(String nombre, int dni, int anioIngreso, double sueldoBasico) {
        this.nombre = nombre;
        this.dni = dni;
        this.anioIngreso = anioIngreso;
        this.sueldoBasico = sueldoBasico;
    }
    
    

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public int getDni() {
        return dni;
    }

    public void setDni(int dni) {
        this.dni = dni;
    }

    public int getAnioIngreso() {
        return anioIngreso;
    }

    public void setAnioIngreso(int anioIngreso) {
        this.anioIngreso = anioIngreso;
    }

    public double getSueldoBasico() {
        return sueldoBasico;
    }

    public void setSueldoBasico(double sueldoBasico) {
        this.sueldoBasico = sueldoBasico;
    }
  
    
    public abstract double sueldoACobrar();
    
    public String toString (){
        
        String aux;
        aux=  "Nombre : " + this.getNombre() +
                " DNI : " +  this.getDni() +" SUELDO: " + this.sueldoACobrar();
        return aux;
    }
    

}
