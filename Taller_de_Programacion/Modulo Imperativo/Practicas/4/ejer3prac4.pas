program ejer3prac4;
type



    producto = record
        codeProducto:integer;
        cantTotalUniVendidas:integer;
        montoTotal:integer;
    end;
    
    
    venta = record
        codeVenta:integer;
        codeProductoVendido:integer;
        cantUnidadesVendidas:integer;
        precioUnitario:integer;
    end;



    arbol=^nodo;

    nodo = record
        elem:producto;
        hI:arbol;
        hD:arbol;
    end;


    procedure generarListaArbol(var a:arbol);

            procedure leerVenta(var v:venta);
            begin
                writeln('ingrese codigo de venta');
                readln(v.codeVenta);
                if(v.codeVenta <> -1) then  begin
                writeln('ingrese codigo del producto vendido');
                readln(v.codeProductoVendido);
                writeln('ingrese la cantidad de unidades vendidas');
                readln(v.cantUnidadesVendidas);
                writeln('ingrese el precio unitario del producto');
                readln(v.precioUnitario);
                end;
            end;


            procedure llenarListaArbol(var a:arbol; v:venta);
            begin
                if (a = nil) then begin
                    new(a); a^.elem.codeProducto:= v.codeProductoVendido; a^.elem.cantTotalUniVendidas:=v.cantUnidadesVendidas; a^.elem.montoTotal:=(v.cantUnidadesVendidas * v.precioUnitario);
                    a^.hI:=nil; a^.hD:=nil;
                end
                else
                begin
                    if(v.codeProductoVendido < a^.elem.codeProducto) then llenarListaArbol(a^.hI,v)
                    else 
                    if(v.codeProductoVendido = a^.elem.codeProducto) then
                    begin
                        a^.elem.cantTotalUniVendidas:=(a^.elem.cantTotalUniVendidas + v.cantUnidadesVendidas);
                        a^.elem.montoTotal:=a^.elem.montoTotal + (v.cantUnidadesVendidas * v.precioUnitario);
                    end
                    else if(v.codeProductoVendido > a^.elem.codeProducto) then llenarListaArbol(a^.hD,v)
                end;
            end;
    var 
    v:venta;
    begin
        leerVenta(v);
        if(v.codeVenta <> -1) then begin
            llenarListaArbol(a,v);
            generarListaArbol(a);
        end;  
    end;
    procedure imprimirArbol(a:arbol);
    begin
        if(a<>nil) then begin
            writeln('------------------------------');
            writeln('Codigo: ',a^.elem.codeProducto);
            writeln('Monto Total: ',a^.elem.montoTotal);
            writeln('Cantidad Unidades: ', a^.elem.cantTotalUniVendidas);
            writeln('------------------------------');
            imprimirArbol(a^.hI);
            imprimirArbol(a^.hD);
        end;
    end;

    procedure mayorUnidadesVendidas(a:arbol);

        function calcularMax(a:arbol; var max:integer; var codemax:integer):integer;
        begin
            if(a<>nil) then begin
                if(a^.elem.cantTotalUniVendidas > max) then begin
                    max:=a^.elem.cantTotalUniVendidas;
                    codemax:=a^.elem.codeProducto;
                end;
                calcularMax:=calcularMax(a^.hI,max,codemax);
                calcularMax:=calcularMax(a^.hD,max,codemax);
            end;
        end;

    var
    max:integer;
    codemax:integer;
    begin
        max:=-999;
        calcularMax(a,max,codemax);
        writeln;
        writeln('----------------------------------------------------------------------------');
        writeln('el codigo del producto con mayor cantida de unidades venidas fue:', codemax);
        writeln('----------------------------------------------------------------------------');
        writeln;
    end;

    procedure CantDeCodesMenoresAX(a:arbol; x:integer);

        function contarMenores(a:arbol; x:integer; var cantMenores:integer):integer;
        begin
            if(a<>nil) then begin
                if(a^.elem.codeProducto < x) then 
                    cantMenores:=cantMenores + 1;
                contarMenores(a^.hI,x,cantMenores);
                contarMenores(a^.hD,x,cantMenores);
            end;
            contarMenores:=cantMenores;
        end;
    var
    cantMenores:integer;
    begin
        cantMenores:=0;
        contarMenores(a,x,cantMenores);
        writeln('----------------------------------------------------------------------------');
        writeln('la cantidad de codigos que existen menores a el nuemero fueron:', cantMenores);
        writeln('----------------------------------------------------------------------------');
        writeln;
    end;

    procedure MontoEntreDosValores(a:arbol; min:integer; max:integer; var monto:integer);
    begin
        if(a <> nil) then begin
            if(a^.elem.codeProducto > min)
            and (a^.elem.codeProducto < max) then
            monto:=(monto + a^.elem.montoTotal);
            MontoEntreDosValores(a^.hI,min,max,monto);
            MontoEntreDosValores(a^.hD,min,max,monto);
        end;
    end;



var
a:arbol;
x:integer;
valor1:integer;
valor2:integer;
montoTotalentre:integer;
min:integer;
max:integer;
begin
    montoTotalentre:=0;
    a:=nil;
    generarListaArbol(a);
    imprimirArbol(a);
    mayorUnidadesVendidas(a);
    writeln;
    writeln('ingrese numero');
    readln(x);
    CantDeCodesMenoresAX(a,x);
    writeln;
    writeln('ingrese el valor 1');
    readln(valor1);
    writeln;
    writeln('ingrese el valor 2');
    readln(valor2);
    if(valor1< valor2)then begin
        min:=valor1;
        max:=valor2;
    end
    else begin
        min:=valor2;
        max:=valor1;
    end;

    MontoEntreDosValores(a,min,max, montoTotalentre);

    writeln('----------------------------------------------------------------------------');
    writeln('el monto total es:',montoTotalentre);
    writeln('----------------------------------------------------------------------------');
end.


