{2. Una agencia dedicada a la venta de autos ha organizado su stock y, tiene la información de los autos en venta. Implementar un programa que:
 a) Genere la información de los autos (patente, año de fabricación (2010..2018), marca y modelo, finalizando con marca ‘MMM’)
 y los almacene en dos estructuras de datos: 
 i. Una estructura eficiente para la búsqueda por patente.
 ii. Una estructura eficiente para la búsqueda por marca. Para cada marca se deben almacenar todos juntos los autos pertenecientes a ella.
 b) Invoque a un módulo que reciba la estructura generado en a) i y una marca y retorne la cantidad de autos de dicha marca que posee la agencia.
 c)Invoque a un módulo que reciba la estructura generado en a) ii y una marca y retorne la cantidad de autos de dicha marca que posee la agencia.
 d) Invoque a un módulo que reciba el árbol generado en a) i y retorne una estructura con la información de los autos agrupados por año de fabricación
 e) Invoque a un módulo que reciba el árbol generado en a) i y una patente y devuelva el modelo del auto con dicha patente.
 f)Invoque a un módulo que reciba el árbol generado en a) ii y una patente y devuelva el modelo del auto con dicha patente.
}


program ejer2prac5;
type

    dataAnio = 2010.. 2018;

    auto = record   
        patente:integer;
        anioFabricacion:dataAnio;
        marca:string;
        modelo:string;
    end;
    
    autosLista = record   
        patente:integer;
        anioFabricacion:dataAnio;
        modelo:string;
    end;

    arbol1 = ^nodo1;

    nodo1 = record
        elem:auto;
        hI:arbol1;
        hD:arbol1;
    end;

    listadeAutos =^nodoAutos;

    nodoAutos = record
        elem:autosLista;
        sig:listadeAutos;
    end;

    arbol2 = ^nodo2;

    nodo2 = record
        marca:string;
        lista:listadeAutos;
        hI:arbol2;
        hD:arbol2;
    end;

    autos3 = record  
        patente:integer;
        marca:string;
        modelo:string;
    end;

    lista3 = ^nodolista3;

    nodolista3 = record
        elem:autos3;
        sig:lista3;
    end;

    arbol3 = ^nodo3;

    nodo3 = record
        anioFabricacion:dataAnio;
        lista:lista3;
        hI:arbol3;
        hD:arbol3;
    end;

    procedure cargarArboles(var a1:arbol1; var a2:arbol2);

        procedure leerElemento(var elem:auto);
        begin
            writeln('patente:');
            readln(elem.patente);
            writeln('anio de fabricacion:');
            readln(elem.anioFabricacion);
            writeln('marca:');
            readln(elem.marca);
            writeln('modelo:');
            readln(elem.modelo);
        end;

        procedure llenarArbol1(var a1:arbol1; elem:auto);
        begin
            if(a1 = nil) then begin
                new(a1);  a1^.elem:=elem; a1^.hI:=nil; a1^.hD:=nil;
            end
            else
            begin
                if(elem.patente <= a1^.elem.patente) then llenarArbol1(a1^.hI,elem)
                else llenarArbol1(a1^.hD,elem);
            end;
        end;



        procedure llenarArbol2(var a2:arbol2; elem:auto);

                procedure adaptarElem( elem:auto; var elem2:autosLista);
                begin
                        elem2.patente:=elem.patente;
                        elem2.modelo:=elem.modelo;
                        elem2.anioFabricacion:=elem.anioFabricacion;
                end;

                procedure agregarAdelante(var l:listadeAutos; elem:autosLista);
                var
                nuevo:listadeAutos;
                begin
                    new(nuevo); nuevo^.elem:=elem; nuevo^.sig:=nil;
                    if(l=nil) then l:=nuevo
                    else begin
                        nuevo^.sig:=l;
                        l:=nuevo;
                    end;
                end;
                
        var

        elem2:autosLista;

        begin


            if(a2 = nil) then begin
                new(a2); a2^.marca:=elem.marca; a2^.lista:=nil; a2^.hI:=nil; a2^.hD:=nil;
                adaptarElem(elem,elem2);
                agregarAdelante(a2^.lista,elem2);
            end
            else
                if(elem.marca < a2^.marca) then llenarArbol2(a2^.hI,elem)
                else if(elem.marca = a2^.marca) then begin 
                adaptarElem(elem,elem2);
                agregarAdelante(a2^.lista,elem2);
                end
                else
                llenarArbol2(a2^.hD,elem);
        end;

    var
    elem:auto;

    begin
        leerElemento(elem);
        if(elem.marca <> 'MMM') then begin
            llenarArbol1(a1, elem);
            llenarArbol2(a2,elem);
            cargarArboles(a1,a2);
        end;
    end;


{b) Invoque a un módulo que reciba la estructura generado en a) i y una marca y retorne la cantidad de autos de dicha marca que posee la agencia.}
    procedure CantMarcaArbol1(a1:arbol1; marcaR:string);

        function cantidad(a1:arbol1; marcaR:string; var cantTotal:integer):integer;
        begin
            if(a1<>nil) then begin
                if(a1^.elem.marca = marcaR) then
                    cantTotal:= cantTotal + 1;
                cantidad:=cantidad(a1^.hI,marcaR,cantTotal);
                cantidad:=cantidad(a1^.hD,marcaR,cantTotal);
            end;

            cantidad:=cantTotal;

        end;
    var
    cantTotal:integer;
    begin
        cantTotal:=0;
        writeln('la cantidad total:   ',  cantidad(a1,marcaR,cantTotal));
    end;
{c)Invoque a un módulo que reciba la estructura generado en a) ii y una marca y retorne la cantidad de autos de dicha marca que posee la agencia.}

    procedure CantMarcaArbol2(a2:arbol2; marcaR:string);

            
        function cantidad(a2:arbol2; marcaR:string; var cantTotal:integer):integer;


            procedure cuentolista(l:listadeAutos; var cantTotal:integer);
            begin
                while(l<>nil) do begin
                    cantTotal:=cantTotal + 1;
                    l:=l^.sig;
                end;
            end;


        begin

            if(a2<>nil) then begin
                if(marcaR < a2^.marca) then cantidad:=cantidad(a2^.hI,marcaR,cantTotal)
                else if(marcaR = a2^.marca) then cuentolista(a2^.lista,cantTotal)
                else cantidad:=cantidad(a2^.hD,marcaR,cantTotal);
            end;
            cantidad:=cantTotal;
        end;

    var
    cantTotal:integer;
    begin
        cantTotal:=0;
        writeln('la cantidad total:   ',  cantidad(a2,marcaR,cantTotal));
    end;



    procedure cargarArbol3(var a3:arbol3; a1:arbol1);
        procedure llenarArbol3(var a3:arbol3; a1:arbol1);
            procedure agregarAdelante(l:lista3; elem:auto);
                procedure adaptarElem(var elem2:autos3; elem:auto);
                begin
                    elem2.patente:=elem.patente;
                    elem2.marca:=elem.marca;
                    elem2.modelo:=elem.modelo;
                end;
            var
            nuevo:lista3;
            elem2:autos3;
            begin
                adaptarElem(elem2,elem);
                new(nuevo); nuevo^.elem:=elem2; nuevo^.sig:=nil;
                if(l= nil) then l:=nuevo
                else begin
                    nuevo^.sig:=l;
                    l:=nuevo;
                end;
            end;
        begin
            if(a3 = nil) then begin
                new(a3); a3^.anioFabricacion:=a1^.elem.anioFabricacion; a3^.lista:=nil; a3^.hI:=nil; a3^.hD:=nil;
                agregarAdelante(a3^.lista, a1^.elem)
            end
            else
                if(a1^.elem.anioFabricacion < a3^.anioFabricacion ) then llenarArbol3(a3^.hI,a1)
                else if (a1^.elem.anioFabricacion = a3^.anioFabricacion) then agregarAdelante(a3^.lista, a1^.elem) 
                else llenarArbol3(a3^.hD,a1)
        end;
    begin
       if(a1<>nil)then begin
            llenarArbol3(a3,a1);
            cargarArbol3(a3,a1^.hI);
            cargarArbol3(a3,a1^.hD);
       end;
    end;

    procedure idemPatente1(a1:arbol1; patenteR:integer; var marcapatente:string);
    begin
        if(a1<>nil) then begin
            if (patenteR < a1^.elem.patente) then idemPatente1(a1^.hI, patenteR, marcapatente)
            else if (patenteR = a1^.elem.patente) then marcapatente:=a1^.elem.marca
            else idemPatente1(a1^.hD, patenteR, marcapatente)
        end; 
    end;

    procedure idemPatente2(a2:arbol2; patenteR:integer; var marcapatente:string);

        function recorrerLista(l:listadeAutos; patenteR:integer; var marcapatente:string):boolean;
        begin
            recorrerLista:=false;
            while (l<>nil) do begin
                if(l^.elem.patente = patenteR) then
                    recorrerLista:=true;
                l:=l^.sig;
            end;
        end;

    begin
        if(a2 <> nil) then begin
            if (recorrerLista(a2^.lista,patenteR,marcapatente)) then
                marcapatente:=a2^.marca;
            idemPatente2(a2^.hI,patenteR,marcapatente);
            idemPatente2(a2^.hD,patenteR,marcapatente);
        end;
    end; 

var
a1:arbol1;
a2:arbol2;
a3:arbol3;
marcaR:string;
patenteR:integer;
marcapatente:string;
begin
    a1:=nil;
    a2:=nil;
    a3:=nil;
    cargarArboles(a1,a2);
    writeln('ingrega marca para buscar la cantidad de autos de dicha marca:  ');
    readln(marcaR);
    CantMarcaArbol1(a1,marcaR);
    writeln;
    writeln('ingrega marca para buscar la cantidad de autos de dicha marca:  ');
    readln(marcaR);
    CantMarcaArbol2(a2,marcaR);
    cargarArbol3(a3,a1);
    writeln;
    writeln('ingrese una patente para saber el modelo');
    readln(patenteR);
    idemPatente1(a1,patenteR,marcapatente);
    writeln(marcapatente);
    writeln;
    writeln('ingrese una patente para saber el modelo');
    readln(patenteR);
    idemPatente2(a2,patenteR,marcapatente);
    writeln(marcapatente);
end.

