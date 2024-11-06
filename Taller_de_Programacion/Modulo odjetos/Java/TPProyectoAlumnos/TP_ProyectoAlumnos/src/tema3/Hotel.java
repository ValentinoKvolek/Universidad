/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema3;

/**
 *
 * @author valen
 */
public class Hotel {
    
    private int  MaxHabitaciones;
    private int diml =0;
    Habitacion [] habitaciones = new Habitacion[MaxHabitaciones];
    
    public Hotel (int n){
        this.MaxHabitaciones = n;
        this.habitaciones = new Habitacion[n];
        for (int i =0; i<n ; i++){
            habitaciones[i] = new Habitacion();
        }
        
    }    

    public int getMaxHabitaciones() {
        return MaxHabitaciones;
    }

    public void setMaxHabitaciones(int MaxHabitaciones) {
        this.MaxHabitaciones = MaxHabitaciones;
    }

    public void agregarCliente(Habitacion nuevaHabitacion,int numHabitacion){
        
        if (!(habitaciones[numHabitacion].isSituacion())){
        
         habitaciones[numHabitacion] = nuevaHabitacion;
        }
        else if (habitaciones[numHabitacion].isSituacion()){
                 System.out.println("ocupado");
                
                 }
    }
    
    public void Aumentar(double monto){
        
        for (int i = 0 ; i < MaxHabitaciones ; i++){
            double aumento = habitaciones[i].getCosto() + monto;
            habitaciones[i].setCosto(aumento);
            
        }
    
    }

    @Override
    public String toString() {
    
        String fermin = "";
      
        for (int i = 0 ; i < MaxHabitaciones ; i++){
            
            fermin =  fermin + "habitacion: " + i + habitaciones[i].getCosto();
            if (habitaciones[i].isSituacion())
                fermin = fermin + "  " + "ocuapada" + habitaciones[i].getNuevocliente();
            
            else {fermin = fermin + "  " + "LIBRE"; }
            
            fermin = fermin + "\n";
            
        }
        
        return fermin;
        
    }
}
