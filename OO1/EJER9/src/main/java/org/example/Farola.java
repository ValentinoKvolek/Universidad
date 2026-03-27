package org.example;
import java.util.ArrayList;
import java.util.List;

public class Farola {

    private boolean estado = false;
    private List<Farola> vecinos = new ArrayList();

    public Farola() {}

    public void pairWithNeighbor( Farola otraFarola){
        if (otraFarola != null){
            vecinos.add(otraFarola);
            otraFarola.vecinos.add(this);
        }
    }
    public List<Farola> getNeighbors(){
        if(vecinos != null){
            return  vecinos;
        }
        return  new ArrayList<>();
    }
    public void turnOn(){
        if(isOff()){
            estado = true;
            for (Farola vecino : vecinos) {
                vecino.turnOn();
            }
        }
    }
    public void turnOff(){
        if(isOn()){
            estado = false;
            for(Farola vecino: vecinos){
                vecino.turnOff();
            }
        }
    }
    public boolean isOn(){
        if (estado){
            return  true;
        }
        return false;
    }
    public boolean isOff(){
        if(estado){
            return false;
        }
        return true;
    }

}
