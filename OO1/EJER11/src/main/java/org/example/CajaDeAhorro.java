package org.example;

public class CajaDeAhorro extends Cuenta {


    protected boolean puedeExtraer(double monto){
        double costoAdicional = monto * 0.02;
        double montoFinal = monto + costoAdicional;
        return (this.getSaldo() - montoFinal )>= 0;
    }





}
