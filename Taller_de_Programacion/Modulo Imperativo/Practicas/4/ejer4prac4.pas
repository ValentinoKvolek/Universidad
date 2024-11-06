program ejer4prac3;
type
    prestamo = record
        ISBN:integer;
        numSocio:integer;
        dia:integer;
        mes:integer;
        cantDias:integer;
    end;

    arbol1 = ^nodo1;

    nodo1 = record  
        elem:prestamo;
        hI:arbol1;
        hD:arbol1;
    end;

    prestamoL=record    //prestamo!
        numSocio:integer;
        dia:integer;
        mes:integer;
        cantDias:integer;
    end;


    listaPrestamo = ^nodoPrestamo;

    nodoPrestamo = record
        elem:prestamoL;
        sig:listaPrestamo;
    end;

    arbol2 = ^nodo2;

    nodo2 = record
        ISBN:integer;
        lista:listaPrestamo;
        hI:arbol2;
        hD:arbol2;
    end;


    arbolF = ^nodof;

    nodof = record
        ISBN:integer;
        cantTotal:integer;
        hI:arbolF;
        hD:arbolF;
    end;

    arbolG = ^nodog;

    nodog = record
        ISBN:integer;
        cantTotal:integer;
        hI:arbolG;
        hD:arbolG;
    end;


procedure generalArboles(var a1:arbol1; var a2:arbol2);

    procedure LeerPrestamo(var e:prestamo);
    begin
        writeln('ingrese el ISBN  :');
        readln(e.ISBN);
        if(e.ISBN <> 0) then begin
            writeln;
            writeln('ingrese el numero de socio  :');
            readln(e.numSocio);
            writeln;
            writeln('ingrese el dia  :');
            readln(e.dia);
            writeln;
            writeln('ingrese el mes  :');
            readln(e.mes);
            writeln;
            writeln('ingrese el cantidad de dias que se presto :');
            readln(e.cantDias);
            writeln;
        end;
    end;

    procedure llenarArbol1(var a1:arbol1; elem:prestamo);
    begin
        if (a1 = nil) then begin
            new(a1); a1^.elem:=elem; a1^.hI:=nil; a1^.hD:=nil;
        end
        else if(elem.ISBN < a1^.elem.ISBN) then llenarArbol1(a1^.hI,elem)
        else if(elem.ISBN >= a1^.elem.ISBN) then llenarArbol1(a1^.hD, elem)
    end;
    
    procedure llenarArbol2(var a2:arbol2 ; elem:prestamo);

        procedure adaptarRecord(elem:prestamo; var pl:prestamoL);
        begin
            pl.numSocio:=elem.numSocio;
            pl.dia:=elem.dia;
            pl.mes:=elem.mes;
            pl.cantDias:=elem.cantDias;
        end;

        procedure agregarAdelante(var l:listaPrestamo; pl:prestamoL);
        var
        nuevo:listaPrestamo;
        begin
            new(nuevo); nuevo^.elem:=pl; nuevo^.sig:=nil;
            if(l = nil) then l:=nuevo
            else begin
                nuevo^.sig:=l;
                l:=nuevo;
            end;
        end;

    var
    pl:prestamoL;
    begin
        adaptarRecord(elem,pl);
        if(a2 = nil) then begin
            new(a2); a2^.ISBN:=elem.ISBN; a2^.lista:=nil; a2^.hI:=nil; a2^.hD:=nil;
            agregarAdelante(a2^.lista,pl);
        end
        else if (elem.ISBN < a2^.ISBN) then llenarArbol2(a2^.hI,elem)
        else if(elem.ISBN = a2^.ISBN) then agregarAdelante(a2^.lista,pl)
        else  if (elem.ISBN > a2^.ISBN ) then llenarArbol2(a2^.hD,elem)
    end;



var
    elem:prestamo;
begin
    LeerPrestamo(elem);
    if(elem.ISBN <> 0) then begin
        llenarArbol1(a1,elem);
        llenarArbol2(a2,elem);
        generalArboles(a1,a2);
    end;
end;


procedure IsbnMasGrande(a1:arbol1);

    procedure buscarMax(a1:arbol1; var max:integer);
    begin
        if(a1<>nil) then begin
            if(a1^.elem.ISBN > max) then 
                max:=a1^.elem.ISBN;
            buscarMax(a1^.hI,max);
            buscarMax(a1^.hD,max);
        end;
    end;
var
max:integer;
begin
    max:=-99;
    buscarMax(a1,max);
    writeln('------------------------------');
    writeln('el isbn mas grande fue : ', max);
    writeln('------------------------------');

end;

procedure IsbnMasPequenio(a2:arbol2);
    procedure buscarMin(a2:arbol2; var min:integer);
    begin
         if(a2<>nil) then begin
            if(a2^.ISBN < min) then 
                min:=a2^.ISBN;
            buscarMin(a2^.hI,min);
            buscarMin(a2^.hD,min);
        end;
    end;
var
    min:integer;
begin
    min:=99;
    buscarMin(a2,min);
    writeln('------------------------------');
    writeln('el isbn mas chico fue : ', min); 
    writeln('------------------------------');

end;


procedure cantDePrestamosRealizadosA1(a1:arbol1; num:integer; var cantp:integer);

    procedure contarPrestamos(a1:arbol1; num:integer; var cantp:integer);
    begin
        if(a1<>nil) then begin
            if(a1^.elem.numSocio = num) then 
                cantp:= cantp + 1;
            contarPrestamos(a1^.hI,num,cantp);
            contarPrestamos(a1^.hD,num,cantp);
        end;
    end;
begin
    contarPrestamos(a1,num,cantp);
end;


procedure cantDePrestamosRealizadosA2(a2:arbol2; socio:integer; var cantp:integer);

    procedure contarPrestamosa2(l:listaPrestamo; socio:integer; var cantp:integer);
    begin
        if (l<>nil) then begin
            if (l^.elem.numSocio = socio) then
                cantp:=cantp + 1;
            l:=l^.sig;
        end;
    end;
begin
    if(a2<>nil) then begin
        contarPrestamosa2(a2^.lista,socio,cantp);
        cantDePrestamosRealizadosA2(a2^.hI,socio,cantp);
        cantDePrestamosRealizadosA2(a2^.hD,socio,cantp);
    end;
end;


procedure generarArbolF(var af:arbolF; a1:arbol1);
begin
    if(af = nil) then begin
        new(af); af^.ISBN:= a1^.elem.ISBN; af^.cantTotal:= 1; af^.hI:=nil; af^.hD:=nil;
    end
    else
        if(a1^.elem.ISBN < af^.ISBN) then generarArbolF(af^.hI,a1) else
        if(a1^.elem.ISBN = af^.ISBN) then af^.cantTotal:= af^.cantTotal + 1;
        if(a1^.elem.ISBN > af^.ISBN) then generarArbolF(af^.hD,a1)
end;

procedure generarArbolG(var ag:arbolG; a2:arbol2);


    function contar(l:listaPrestamo):integer;
    begin
        contar:=0;
        while (l<>nil) do begin
            contar:=contar +1;
            l:=l^.sig;
        end;

    end;

    procedure insertar(var ag:arbolG; a2:arbol2);
    begin
            if(ag = nil) then begin
            new(ag); ag^.ISBN:= a2^.ISBN; ag^.cantTotal:=contar(a2^.lista); ag^.hI:=nil; ag^.hD:=nil;
        end
        else
            if(a2^.ISBN < ag^.ISBN) then generarArbolG(ag^.hI,a2) else
            if(a2^.ISBN = ag^.ISBN) then ag^.cantTotal:= ag^.cantTotal + contar(a2^.lista);
            if(a2^.ISBN > ag^.ISBN) then generarArbolG(ag^.hD,a2)
    end;

begin
    if(a2 <> nil) then begin 
        insertar(ag,a2);
        generarArbolG(ag,a2^.hI);
        generarArbolG(ag,a2^.hD);
    end;
end;

procedure imprimirG(ag:arbolG);
begin
    if (ag <> nil) then begin
        imprimirG(ag^.hI);
        imprimirG(ag^.hD);
        writeln('------------------------------'); 
        writeln(ag^.ISBN);
        writeln(ag^.cantTotal);
        writeln('------------------------------');
    end;
end;

procedure cantPrestamosTotalesA1(a1:arbol1;max:integer; min:integer);

    procedure contarPrestamos(a1:arbol1; max:integer; min:integer; var cantTotal:integer); 
    begin
        if(a1<>nil) then begin
            if(a1^.elem.ISBN > min) and (a1^.elem.ISBN < max) then
                cantTotal:= (cantTotal +1);
            contarPrestamos(a1^.hI,max,min,cantTotal);
            contarPrestamos(a1^.hD,max,min,cantTotal);
        end;
    end;
var
cantTotal:integer;
begin
    cantTotal:=0;
    contarPrestamos(a1,max,min,cantTotal);
    writeln('------------------------------');
    writeln('los prestamos que se hicieron entre  :', min , '  y  :', max , '  son  :', cantTotal);
    writeln('------------------------------');
end;


procedure cantPrestamosTotalesA2(a2:arbol2; max:integer; min:integer);

    procedure contarPrestamosa2(a2:arbol2; max:integer; min:integer; var cantTotal:integer); 
    begin
        if(a2<>nil) then begin
            if(a2^.ISBN > min) and (a2^.ISBN < max) then
                cantTotal:= cantTotal +1;
            contarPrestamosa2(a2^.hI,max,min,cantTotal);
            contarPrestamosa2(a2^.hD,max,min,cantTotal);
        end;
    end;
var
cantTotal:integer;
begin
    cantTotal:=0;
    contarPrestamosa2(a2,max,min,cantTotal);
    writeln('------------------------------');
    writeln('los prestamos que se hicieron entre  :', min , '  y  :', max , '  son  :', cantTotal);
    writeln('------------------------------');
end;

var
a1:arbol1;
a2:arbol2;
socio:integer;
cantp:integer;
af:arbolF;
ag:arbolG;
isbm1:integer;
isbm2:integer;
max:integer;
min:integer;
begin
    cantp:=0;
    generalArboles(a1,a2);
    IsbnMasGrande(a1);
    IsbnMasPequenio(a2);
    writeln;
    writeln('------------------------------');
    writeln('ingrese un numero de socio');
    writeln('------------------------------');
    readln(socio);
    cantDePrestamosRealizadosA1(a1,socio,cantp);
    writeln('------------------------------');
    writeln('la cantidad de préstamos realizados a dicho socio fue:',cantp);
    writeln('------------------------------');
    writeln('------------------------------');
    cantp:=0;
    writeln('ingrese un numero de socio');
    writeln('------------------------------'); 
    readln(socio);
    cantDePrestamosRealizadosA2(a2,socio,cantp);
    writeln('------------------------------');
    writeln('la cantidad de préstamos realizados a dicho socio fue:',cantp);
    writeln('------------------------------');
    generarArbolF(af,a1);
    generarArbolG(ag,a2);
    imprimirG(ag);
    writeln('ingrese un numero de isbm');
    writeln('------------------------------'); 
    readln(isbm1);
    writeln('ingrese un segundo de isbm');
    writeln('------------------------------'); 
    readln(isbm2);
        if (isbm1 > isbm2) then begin
            max:=isbm1;
            min:=isbm2;
        end
        else if (isbm1 < isbm2) then begin
            max:=isbm2;
            min:=isbm1;
        end;
    cantPrestamosTotalesA1(a1,max,min);
    cantPrestamosTotalesA2(a2,max,min);
end.