package org.example;

import java.util.List;

public class PedidoArticuloSuper  extends  Pedido{

    boolean cuponDescuento;

    public PedidoArticuloSuper(String email, MetodoEnvio envio, TipoPedido tipo, boolean cupon) {
        super(email, envio, tipo);
        this.cuponDescuento = cupon;
    }

    @Override
    public boolean agregar (Producto p){
        if (p instanceof ArticuloSupermercado){
            ArticuloSupermercado x = (ArticuloSupermercado) p;
            if (x.getPesoKG() <= 35) {
                return  super.agregar(p);
            }
            return  false;
        }
        return false;
    }

}
