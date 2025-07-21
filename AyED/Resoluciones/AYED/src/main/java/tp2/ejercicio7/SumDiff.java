package main.java.tp2.ejercicio7;

public class SumDiff {

    private int suma =0 ;
    private int diferencia;

    public SumDiff(int suma, int diferencia) {
        this.suma = suma;
        this.diferencia = diferencia;
    }

    @Override
    public String toString() {
        return "(" + suma + ", " + diferencia + ")";
    }

    public int getSuma() {
        return suma;
    }

    public void setSuma(int suma) {
        this.suma = suma;
    }

    public int getDiferencia() {
        return diferencia;
    }

    public void setDiferencia(int diferencia) {
        this.diferencia = diferencia;
    }
}
