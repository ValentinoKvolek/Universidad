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
public class Dibujo {
       
private String titulo;
private Figura [] vector;
private int guardadas= 0; //diml
private int capacidadMaxima=10;


//inicia el dibujo, sin figuras
public Dibujo (String titulo){
    setTitulo(titulo);
    setVector(new Figura[getCapacidadMaxima()]);
}

    public Figura[] getVector() {
        return vector;
    }

    public void setVector(Figura[] vector) {
        this.vector = vector;
    }

    public int getGuardadas() {
        return guardadas;
    }

    public void setGuardadas(int guardadas) {
        this.guardadas = guardadas;
    }

    public int getCapacidadMaxima() {
        return capacidadMaxima;
    }

    public void setCapacidadMaxima(int capacidadMaxima) {
        this.capacidadMaxima = capacidadMaxima;
    }
    
    public String getTitulo() {
        return titulo;
    }

    public void setTitulo(String titulo) {
        this.titulo = titulo;
    }
    
//agrega la figura al dibujo

    public void agregar(Figura f){

        vector[getGuardadas()] = f;
        guardadas ++;
        System.out.println("la figura" +
        f.toString() +
        "se guardo!.");
    }

    //calcula el área del dibujo:
    //suma de las áreas de sus figuras
    public double calcularArea(){
        double sumaAreas=0;
        for (int i = 0; i < getGuardadas(); i++ ){
            sumaAreas  =  sumaAreas + vector[i].calcularArea();
        }
        
        return sumaAreas;
    }
    
    //imprime el título, representación
    //de cada figura, y área del dibujo
    public void mostrar(){
         for (int i = 0; i < getGuardadas(); i++ ){
             System.out.println( " el titulo del dibuejo es : " + getTitulo()+ " esta constituido por :" + vector[i].toString()+ " y el area total : " + calcularArea());
        }
         
    }
    
    //retorna está lleno el dibujo
    public boolean estaLleno() {
    return (guardadas == capacidadMaxima);
    }
    }