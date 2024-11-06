program final1;

type
subrangopagos:[1,2] {1 efectivo, 2 tarjeta}
    ventas = record
        numeroVentas:integer;
        cantProductos:integer;
        tipoPago:subrangopagos;
    end;

    lista = ^nodo;

    nodo = record  
        elem : ventas;
        sig:lista;
    end;
    
procedure cargarLista1(var l:lista);
begin
    {se dispone}
end;

procedure cargarLista2(l:lista; var l2:lista);
var
nuevo:lista;
begin
    while (l<>nil) do begin
        if (maspares(l^.elem.cantProductos)) do
            insertarOrdenado(l2,l^.elem);
        end;
    end;
end;


function maspares(num:integer):boolean;
var
dig:integer;
cantpares, cantimpares: integer;
begin
cantimpares:=0;
cantpares:=0;
maspares:=false;
    while (num<>0) do begin
        dig:=num mod 10;
        if(dig mod 2 = 0) then 
            cantpares:=cantpares +1;
        else
        begin
            cantimpares:= cantimpares +1
        end;
        num:= num div 10;
    end;
    if (cantpares>cantimpares) then 
        maspares:=true;
end;

procedure insertarOrdenado(var l2:lista; datos:lista);
var
actual,ant,nuevo:lista;
begin
    new(nuevo); nuevo^.elem:=dato; nuevo^.sig:= nil;
    if (l2= nil) then l2 := nuevo;
    else
    begin
        actual:=l2;
        ant:=l2;
        while(l2<>nil) and (nuevo^.elem.tipoPago < actual^.elem.tipoPago) then begin
            ant:= actual;
            actual:= actual^.sig;
        end;
        else
        begin
            if(l2 = actual) then begin
                nuevo^.sig:=l2;
                l2:=nuevo;
            end
            else
            begin
                ant^.sig:=nuevo;
                nuevo^.sig:= actual;
            end;
        end;
    end;
end;