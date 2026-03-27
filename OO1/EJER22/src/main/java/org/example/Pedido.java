package org.example;

import javax.swing.*;
import java.util.List;




public abstract class Pedido {

    enum TipoPedido {
        VIANDAS,
        SUPERMERCADO
    }

    private String email;
    private List<Producto> productos;
    private TipoPedido tipo;
    private MetodoEnvio envio;

    public Pedido(String email, MetodoEnvio envio, TipoPedido tipo) {
        this.email = email;
        this.envio = envio;
        this.tipo = tipo;
    }


    public double calcularCosto(){
        return productos.stream().map(p -> p.costo()).mapToDouble(Double::doubleValue).sum();
    }

    public List<Producto> getProductos() {
        return productos;
    }

    public TipoPedido getTipo() {
        return tipo;
    }

    public  boolean agregar(Producto p){
       return this.productos.add(p);
    }
}
