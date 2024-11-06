{3. Implementar un programa que contenga:
a. Un módulo que lea información de los finales rendidos por los alumnos de la Facultad de Informática y los almacene en una estructura de datos.
La información que se lee es legajo, código de materia, fecha y nota.La lectura de los alumnos finaliza con legajo 0.
La estructura generada debe ser eficiente para la búsqueda por número de legajo y para cada alumno deben guardarse los finales que rindió en una lista. 
b. Un módulo que reciba la estructura generada en a. y retorne la cantidad de alumnos con legajo impar.
c. Un módulo que reciba la estructura generada en a. e informe, para cada alumno, su legajo y su cantidad de finales aprobados (nota mayor o igual a 4)
c. Un módulo que reciba la estructura generada en a. y un valor real.
Este módulo debe retornar los legajos y promedios de los alumnos cuyo promedio supera el valor ingresado.}
program ejer3prac3;
type

    rangoDia = 1.. 31;
    rangoMes = 1.. 12;
    rangoAnio = 2020.. 2024;

    datafecha = record
        dia:rangoDia;
        mes:rangoMes;
        anio:rangoAnio;
    end;

    final = record
        legajo:integer;
        codeMateria:integer;
        fecha:datafecha;
        nota:real;
    end;

    finalArbol = record
        codeMateria:integer;
        fecha:datafecha;
        nota:real;
    end;
    
    listaFinales = ^nodoFinales;
    
    nodoFinales = record
        elem:finalArbol;
        sig:listaFinales;
    end;

    arbol=^nodo;

    nodo = record
        legajo:integer;
        lista:listaFinales;
        hI:arbol;
        hD:arbol;
    end;




procedure generarListaArbol(var a:arbol);

    procedure leerElemento(var e:final);
    begin
        writeln('ingrese el legajo:');
        writeln;
        readln(e.legajo);
        writeln('ingrese el codigo de materia:');
        writeln;
        readln(e.codeMateria);
        writeln('ingrese el dia:');
        writeln;
        readln(e.fecha.dia);
        writeln('ingrese el mes:');
        writeln;
        readln(e.fecha.mes);
        writeln('ingrese el anio:');
        writeln;
        readln(e.fecha.anio);
    end;

    procedure agregarAdelante(var l:listaFinales; finalNuevo:finalArbol);
    var
    nuevo:listaFinales;
    begin
        new(nuevo); nuevo^.elem:=finalNuevo; nuevo^.sig:=nil;
        if(l = nil) then nuevo:=l
        else 
        begin
            nuevo^.sig:=l;
            l:=nuevo;
        end;
    end;

    procedure llenarListaArbol(var a:arbol; dato:final);

        procedure FinalParaArbol(f:final; var fa:finalArbol);
        begin
            fa.codeMateria:=f.codeMateria;
            fa.fecha.dia:=f.fecha.dia;
            fa.fecha.mes:=f.fecha.mes;
            fa.fecha.anio:=f.fecha.anio;
            fa.nota:= f.nota;
        end;


    var
    fa:finalArbol;
    begin
        FinalParaArbol(dato,fa);
        if (a = nil) then begin
            new(a); a^.legajo:=dato.legajo; a^.lista:=nil; a^.hi:=nil; a^.hd:=nil;
            agregarAdelante(a^.lista,fa); {agregar el primer final}
        end
            else if (a^.legajo < dato.legajo) then llenarListaArbol(a^.hi,dato)
            else if(a^.legajo = dato.legajo) then agregarAdelante(a^.lista,fa) {si mi legajo es igual quiere decir que estamos hablando del mismo alumno}
            else llenarListaArbol(a^.hD,dato);
    end;

var

    elemento:final;
    
begin
    leerElemento(elemento);
    if(elemento.legajo <> 0) then begin
        llenarListaArbol(a,elemento);
        leerElemento(elemento);
    end;
end;

{b. Un módulo que reciba la estructura generada en a. y retorne la cantidad de alumnos con legajo impar.}

procedure legajosImpares(a:arbol);

    function Comprobar(num:integer):boolean;
    var
    dig:integer;
    begin                               
        Comprobar:=false;
        dig:=num mod 10;
        if (dig mod 2 <> 0) then 
                comprobar := true;
    end;

    procedure ContarIm(a:arbol; var actual:integer; var cantImpares:integer);
    begin
        
    if(a<>nil) then begin         
            if(Comprobar(a^.elem.legajo)) then  {antes de sumar el legajo verificar que no es el mismo alumno }
                cant:= cant + 1
        end;
        ContarIm(a^.hI,cant);
        ContarIm(a^.hD,cant);
        end;
    end;
var
actualLegajo:integer;
cantImpares:integer;
begin
    cantImpares:=0;
    actualLegajo:=0;
    ContarIm(a,actualLegajo,cantImpares);
    writeln(cantImpares);
end;


{c. Un módulo que reciba la estructura generada en a. e informe, para cada alumno, su legajo y su cantidad de finales aprobados (nota mayor o igual a 4)}

procedure cantidadFinales(a:arbol);

    function aprobo(l:listaFinales):integer;
    begin
        while (l<>nil) do begin
            if (l^.elem.nota >= 4) then begin
                aprobo:=aprobo +1;
            l:=l^.sig;
        end;
    end;

    procedure alumnosNotas (a:arbol; var cantAprobados:integer);
    begin
        if (a<>nil) then 
        begin
        aprobo(a^.listaFinales);
        alumnosNotas(a^.hI, cantAprobados);
        alumnosNotas(a^.hD, cantAprobados);
        end;
    end;



var
cantAprobados:integer;
actualLegajo:integer;
begin
    cantAprobados:=0;
    actualLegajo:=0;
    if (a<>nil) then
    begin
        alumnosNotas(a,cantAprobados,actualLegajo);
    end;

end;

{programa principal}
var
a:arbol;
begin
    cantAprobados:=0;
    cantImpares:=0;
    generarListaArbol(a);
    legajosImpares(a);
    cantidadFinales(a, cantAprobados);
    writeln(cantAprobados);
end.