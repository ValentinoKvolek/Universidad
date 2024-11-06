/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package prac2prac5;

/**
 *
 * @author valen
 */
public class Estacionamiento {
    
    private String nombre;
    private String direccion;
    private int horaApertura;
    private int horaCierre;
    private int numPiso;
    private int numPlaza;
    
    private Auto[][] matriz;

    public Estacionamiento(String nombre, String direccion) {
        this.setNombre(nombre);
        this.setDireccion(direccion);
        this.setHoraApertura(5);
        this.setHoraCierre(21);
        this.numPiso = 5;
        this.numPlaza = 10;
        this.setMatriz(new Auto[5-1][10-1]);
    }

    public Estacionamiento(String nombre, String direccion, int horaApertura, int horaCierre, int numPiso, int numPlaza) {
        this.setNombre(nombre);
        this.setDireccion(direccion);
        this.setHoraApertura(horaApertura);
        this.setHoraCierre(horaCierre);
        this.setNumPiso(numPiso);
        this.setNumPlaza(numPlaza);
        this.setMatriz(new Auto [getNumPiso()][getNumPlaza()]);
    }
    

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public String getDireccion() {
        return direccion;
    }

    public void setDireccion(String direccion) {
        this.direccion = direccion;
    }

    public int getHoraApertura() {
        return horaApertura;
    }

    public void setHoraApertura(int horaApertura) {
        this.horaApertura = horaApertura;
    }

    public int getHoraCierre() {
        return horaCierre;
    }

    public void setHoraCierre(int horaCierre) {
        this.horaCierre = horaCierre;
    }

    public int getNumPiso() {
        return numPiso;
    }

    public void setNumPiso(int numPiso) {
        this.numPiso = numPiso;
    }

    public int getNumPlaza() {
        return numPlaza;
    }

    public void setNumPlaza(int numPlaza) {
        this.numPlaza = numPlaza;
    }

    public Auto[][] getMatriz() {
        return matriz;
    }

    public void setMatriz(Auto[][] matriz) {
        this.matriz = matriz;
    }
    
  public void agregarAuto(Auto autoParaAgregar, int pisoIngresado, int plazaIngresada) {
        pisoIngresado--;
        plazaIngresada--;
        if (pisoIngresado >= 0 && pisoIngresado < numPiso && plazaIngresada >= 0 && plazaIngresada < numPlaza) {
            if (matriz[pisoIngresado][plazaIngresada] == null) {
                matriz[pisoIngresado][plazaIngresada] = autoParaAgregar;
            }
        }
    }
    
public String buscarAuto(int patenteIngresada) {
    int i = 0;
    int j = 0;
    boolean encontrado = false;

    while (i < numPiso && !encontrado) {
        while (j < numPlaza && !encontrado) {
            if (matriz[i][j] != null && matriz[i][j].getPatente() == patenteIngresada) {
                encontrado = true;
            } else {
                j++;
            }
        }
        if (!encontrado) {
            j = 0; // Reiniciar j para la siguiente fila
            i++;
        }
    }

    if (encontrado) {
        return "Auto encontrado en Piso: " + (i + 1) + ", Plaza: " + (j + 1);
    } else {
        return "Auto Inexistente";
    }
}

    
    public void informarEstacionamiento(){
      
        for (int i = 0 ; i < getNumPiso(); i++){
            
            for (int j = 0 ; j < getNumPlaza(); j++){
                
                if(this.matriz[i][j] != null){
                    
                    System.out.println("PISO: " + (i+1) + " PLAZA : " + (j+1) );
                    System.out.println(this.matriz[i][j].toString());
                }
                else System.out.println("PISO: " + (i+1) + " PLAZA : " + (j+1) + " LIBRE");
                
            }
            
        }    
    }
    
    public int cantAutos(int plazaIngresada){
        int cant;
        cant= 0;
        
        for (int i=0; i < getNumPiso(); i++){
            if(this.matriz[i][plazaIngresada] != null){ 
                cant ++;
                
            }
        } 
        
        return cant;
    }
    
}
