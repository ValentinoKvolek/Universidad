package org.example;

abstract class Cuenta {

    private double saldo;


    public Cuenta(){
        this.saldo =0;
    }
    public double getSaldo(){
        return this.saldo;
    }
    public void Depositar(double monto){
        this.saldo+= monto;
    }
    protected void extraerSinControlar(double monto){
        this.saldo =- monto;
    }

    protected abstract boolean puedeExtraer(double monto);

    public boolean extraer(double monto){
        if(this.puedeExtraer(monto)){
            this.extraerSinControlar(monto);
            return true;
        }
        return false;
    }
    public boolean transferirACuenta(double monto, Cuenta cuentaDestino){
        if(this.puedeExtraer(monto)){
            this.extraerSinControlar(monto);
            cuentaDestino.Depositar(monto);
            return true;
        }
        return false;
    }
}

/*
¿Por qué cree que este ejercicio se llama "Cuenta con ganchos"?
Se llama cuenta con ganchos por que la clase cuenta define el comportamiento general de extraccion y tranferencia, pero
deja un metodo abstracto (puede extraer) como gancho p[ara que cada tipo de cuenta implemente su propia condicion.

En las implementaciones de los métodos extraer() y transferirACuenta()  que se ven en el diagrama, ¿quién es this? ¿Puede decir de qué clase es this?
This es el tipo dinamico del objeto que recibio el mensaje. puede ser tanto caja de ahora como cuenta corriente. el compilador sabe que this es
al menos una cuent. por qu todos los objetos en la jerarquia lo son. esto se llama dynamic dispatch.

¿Por qué decidimos que los métodos puedeExtraer() y extraerSinControlar tengan visibilidad "protegido"?
Esto es por que en las dos clases no tiene sentido que sean publicas. ya que si asi fuera cualquier usuario podria llamarlas
y lel objetivo de las mismas es ser llamdas unicamente para verificacion o despues de esa misma verificacion por la ssubclases.

¿Se puede transferir de una caja de ahorro a una cuenta corriente y viceversa? ¿por qué? ¡Pruébelo!
si se puede tranferir gracias al dinamic dispatch le podemos pasar cualquier tipo de cuenta al metodo.

¿Cómo se declara en Java un metodo abstracto? ¿Es obligatorio implementarlo?
Un metodo abstracto se declara con la palabra abstract, sin cuerpo.
Es obligatorio implementarlo en las clases concretas que hereden de la clase abstracta, salvo que esas clases también sean abstractas

¿Qué dice el compilador de Java si una subclase no implementa un metodo abstracto que hereda?
Class 'CajaDeAhorro' must either be declared abstract or implement abstract method 'puedeExtraer(double)' in 'Cuenta
 */