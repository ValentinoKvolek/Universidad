{a. Implemente un módulo que genere aleatoriamente información de ventas de un comercio.
 Para cada venta generar código de producto, fecha y cantidad de unidades vendidas. 
Finalizar con el código de producto 0. Un producto puede estar en más de una venta. Se pide:}

program ejer2prac3;

type

    dataFechas = record

        dia:integer;
        mes:integer;
        anio:integer;

    end;

    venta = record
        codeProducto:integer;
        fecha:dataFechas;
        UnidadesVendidas:integer;
    end;

    arbol1 = ^nodo1;
    nodo1 = record
        elem:venta;
        hI:arbol1;
        hD:arbol1;
    end;


    ventaArbol2 = record
        codeProducto:integer;
        ventasTotales:integer;
    end;

    arbol2 = ^nodo2;
    nodo2 = record
        elem:ventaArbol2;
        hI:arbol2;
        hD:arbol2;
    end;

    listaDeVentas = ^nodoLista;

    nodoLista = record
        elem:integer;
        sig:listaDeVentas;
    end;

    arbol3 = ^nodo3;
    nodo3 = record
        code:integer;
        lista:listaDeVentas;
        hI:arbol3;
        hD:arbol3;
    end;



procedure GenerarArboles(var a1:arbol1; var a2:arbol2; var a3:arbol3);


    function aleatorioEntre(min:integer; max:integer):integer;
    begin
        aleatorioEntre:=random(max - min + 1) + min;
    end;


    procedure generarVentaALE(var elem:venta);
    begin
        elem.codeProducto:=aleatorioEntre(0,100);
        elem.fecha.dia:=aleatorioEntre(1,31);
        elem.fecha.mes:=aleatorioEntre(1,12);
        elem.fecha.anio:= aleatorioEntre(2001,2024);
        elem.UnidadesVendidas:= aleatorioEntre(1,100);
    end;

    {1 Generar y retornar un árbol binario de búsqueda de ventas ordenado por código de producto. Los códigos repetidos van a la derecha.}

    procedure llenarArbol1(var a1:arbol1; elem:venta);
    begin
        if (a1 = nil) then begin
            new(a1); a1^.elem:=elem; a1^.hI:=nil; a1^.hD:=nil;
        end
        else
        begin
            if(elem.codeProducto < a1^.elem.codeProducto) then llenarArbol1(a1^.hI,elem) 
            else if(elem.codeProducto >= a1^.elem.codeProducto) then llenarArbol1(a1^.hD,elem)
        end;
    end;

{ii. Generar y retornar otro árbol binario de búsqueda de productos vendidos ordenado por código de producto.
 Cada nodo del árbol debe contener el código de producto y la cantidad total de unidades vendidas.}

    procedure llenarArbol2(var a2:arbol2; elem:venta);
    begin
        if(a2 = nil) then begin
            new(a2); a2^.elem.codeProducto:=elem.codeProducto;  a2^.elem.ventasTotales:= elem.UnidadesVendidas; a2^.hI:=nil; a2^.hD:=nil;
        end
        else
        begin
            if(a2^.elem.codeProducto < elem.codeProducto) then llenarArbol2(a2^.hI,elem)
            else if(a2^.elem.codeProducto = elem.codeProducto ) then  a2^.elem.ventasTotales:=(a2^.elem.ventasTotales + elem.UnidadesVendidas)
            else if(a2^.elem.codeProducto > elem.codeProducto) then llenarArbol2(a2^.hD,elem);
        end;
    end;


{iii. Generar y retornar otro árbol binario de búsqueda de productos vendidos ordenado por código de producto.
Cada nodo del árbol debe contener el código de producto y la lista de las ventas realizadas del producto.}

    procedure llenarArbol3(var a3:arbol3; elem:venta);

        procedure agregarAdelante(l:listaDeVentas; num:integer);
        var
        nuevo:listaDeVentas;
        begin
            new(nuevo); nuevo^.elem:=num; nuevo^.sig:=nil;
            iF(l = nil) then l:=nuevo
            else
            begin
                nuevo^.sig:=l;
                l:=nuevo;
            end;
        end;

    begin
        if(a3 = nil) then begin
           new(a3); a3^.code:=elem.codeProducto; a3^.lista:=nil; a3^.hI:=nil; a3^.hD:=nil;
           agregarAdelante(a3^.lista,elem.UnidadesVendidas);
        end
        else
        begin
            if(a3^.code > elem.codeProducto) then llenarArbol3(a3^.hI,elem) 
            else if(a3^.code = elem.codeProducto) then agregarAdelante(a3^.lista,elem.UnidadesVendidas) 
            else if(a3^.code < elem.codeProducto) then llenarArbol3(a3^.hD,elem);
        end;
    end;

var
elem:venta;
begin
    generarVentaALE(elem);
    if(elem.codeProducto <> 0) then begin
        llenarArbol1(a1,elem);
        llenarArbol2(a2,elem);
        llenarArbol3(a3,elem);
        GenerarArboles(a1,a2,a3);
    end;
    writeln('------------------------------');
    writeln('Codigo: ',elem.codeProducto);
    writeln('Cantidad: ', elem.UnidadesVendidas);
    writeln('------------------------------');
end;

var
a1:arbol1;
a2:arbol2;
a3:arbol3;
begin
    randomize;
    GenerarArboles(a1,a2,a3);
end.




