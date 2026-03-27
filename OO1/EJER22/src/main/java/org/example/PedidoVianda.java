package org.example;

import java.util.List;

public class PedidoVianda extends Pedido{
    boolean apto;

    public PedidoVianda(String email, MetodoEnvio envio, TipoPedido tipo, List<Producto> productos, boolean esApto) {
        super(email, envio, tipo);
        this.apto = esApto;
    }

    @Override
    public boolean agregar(Producto p) {
        if(p instanceof Vianda){
            Vianda x =(Vianda) p;
            if (x <= 40){
                if(this.apto == x.aptaCeliaco){
                    return super.agregar(p);
                }
                return false;
            }
            return  false;
        }
        return false;
    }
}
