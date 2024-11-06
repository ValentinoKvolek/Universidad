/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema4;

/**
 *
 * @author valen
 */
public abstract class Empleado {
    
    private  String nombre;
       
    private  double sueldo;
       
    private  int antiguedad;

    public Empleado(String nombre, double sueldo, int antiguedad) {
        this.setNombre(nombre);
        this.setSueldo(sueldo);
        this.setAntiguedad(antiguedad);
    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public double getSueldo() {
        return sueldo;
    }

    public void setSueldo(double sueldo) {
        this.sueldo = sueldo;
    }

    public int getAntiguedad() {
        return antiguedad;
    }

    public void setAntiguedad(int antiguedad) {
        this.antiguedad = antiguedad;
    }
    
    
    public String toString(){
    String aux = "Nombre : " + this.getNombre();       
        return aux;
    }
    
    public abstract double calcularEfectividad();
    
    public abstract double calcularSueldoACobrar();
    
}
