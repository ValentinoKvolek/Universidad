package org.example;

public class ClienteCorporativo extends  Cliente{
    private String nombreEmpresa;
    private String direccion;
    private Integer cuit;

    public ClienteCorporativo() {
    }

    public ClienteCorporativo(String nombreEmpresa, String direccion, Integer cuit) {
        this.nombreEmpresa = nombreEmpresa;
        this.direccion = direccion;
        this.cuit = cuit;
    }

    protected double aplicarDescuento(double total) {
        return total;
    }
}
