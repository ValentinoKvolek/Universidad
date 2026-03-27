package org.example;

public class CuentaCorriente extends Cuenta {

    double limiteDescubierto;

    public CuentaCorriente(){
        this.limiteDescubierto = 0;
    }

    @Override
    protected boolean puedeExtraer(double monto) {
        return (this.getSaldo() - monto >= this.limiteDescubierto);
    }

    public double getDescubierto(){
        return this.limiteDescubierto;
    }
    public void setDescubierto(double descubierto){
        this.limiteDescubierto = descubierto;
    }

}
