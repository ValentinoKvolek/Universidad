package org.example;

import java.time.LocalDate;
import java.util.ArrayList;
import java.util.List;

public class Balanza {


    private Integer cantProductos=0;
    private double precioTotal=0;
    private double pesoTotal=0.0;
    private List<Producto> productos = new ArrayList<>();



    public Balanza(Integer cantProductos, Double precio) {

        this.cantProductos = cantProductos;
        this.precioTotal = precio;
        productos = new ArrayList<>();


    }


    public void ponerEnCero(){
        this.precioTotal = 0.0;
        this.pesoTotal = 0.0;
        this.cantProductos=0;
        productos.clear();
    }


    public void agregarProducto(Producto producto){
        this.cantProductos++;
        this.precioTotal += producto.getPrecio();
        this.pesoTotal += producto.getPeso();
    }

    public Ticket emitirTicket(){

        Ticket ticket = new Ticket();

        ticket.setPesoTotal(this.pesoTotal);
        ticket.setCantidadDeProductos(this.cantProductos);
        ticket.setFecha(LocalDate.now());

        ticket.setPrecioTotal(this.precioTotal);

        ticket.impuesto();

        return ticket;

    }

    public List<Producto> getProductos(){

        return this.productos;

    }


    public Balanza() {}

    public double getPesoTotal() {
        return pesoTotal;
    }

    public Integer getCantProductos() {
        return cantProductos;
    }

    public double getPrecioTotal() {
        return precioTotal;
    }


}
