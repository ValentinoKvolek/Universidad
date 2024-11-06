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
public abstract class Registro{
    
    private int aniosConsecutivos;
    
    private int  anioComienzo;
    
    Estaciones estacion;
    
    private double temperatura;
    
    private double [][] MatrizTemperaturas;
    
    public Registro(int aniosConsecutivos, int anioComienzo, Estaciones estacion, double temperatura) {
        setAniosConsecutivos(aniosConsecutivos);
        setAnioComienzo(anioComienzo);
        setTemperatura(temperatura);
        setEstacion(estacion);
        this.MatrizTemperaturas = new double[aniosConsecutivos][12];
    }

    public double[][] getMatrizTemperaturas() {
        return MatrizTemperaturas;
    }

    public void setMatrizTemperaturas(double[][] MatrizTemperaturas) {
        this.MatrizTemperaturas = MatrizTemperaturas;
    }

    public Estaciones getEstacion() {
        return estacion;
    }

    public void setEstacion(Estaciones estacion) {
        this.estacion = estacion;
    }

    
    

    public int getAniosConsecutivos() {
        return aniosConsecutivos;
    }

    public void setAniosConsecutivos(int aniosConsecurivos) {
        this.aniosConsecutivos = aniosConsecurivos;
    }

    public int getAnioComienzo() {
        return anioComienzo;
    }

    public void setAnioComienzo(int anioComienzo) {
        this.anioComienzo = anioComienzo;
    }

    public double getTemperatura() {
        return temperatura;
    }

    public void setTemperatura(double temperatura) {
        this.temperatura = temperatura;
    }
    
    public void IniciarTemperaturas(){
        for (int i =  this.getAnioComienzo(); i < this.getAniosConsecutivos(); i++){
            for(int j = 0 ; j < 12 ; j++){
                MatrizTemperaturas[i][j] = 800;
            }
        }
    }
    
   public void RegistrarTemperatura(int mes, int anio){
       double temp;
       temp = MatrizTemperaturas[anio][(mes-1)];
       
   }
   
   public double ObtenerTemperatura(int mes ,int anio){
       return MatrizTemperaturas[anio][mes];
   }
    
   public String toString(){
       
        double max = -999;
        int aniomax = 0;
        int mesmax=0;
        
         for (int i =  this.getAnioComienzo(); i < this.getAniosConsecutivos(); i++){
            for(int j = 0 ; j < 12 ; j++){
                if(max < MatrizTemperaturas[i][j]){
                    max = MatrizTemperaturas[i][j];
                    aniomax= i;
                    mesmax= j;
                }
            }
        }
       
        String aux = "el mes : " + (mesmax + 1) +
                        "y anio : " + aniomax +
                            "fueron donde se registro la temperatura maxima";
                       
        return aux;

   }
}
