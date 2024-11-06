{2.- Escribir un programa que:
a. Implemente un módulo recursivo que genere y retorne una lista de números enteros
“random” en el rango 100-200. Finalizar con el número 100.
b. Un módulo recursivo que reciba la lista generada en a) e imprima los valores de la lista en el
mismo orden que están almacenados.
c. Implemente un módulo recursivo que reciba la lista generada en a) e imprima los valores de
la lista en orden inverso al que están almacenados.
d. Implemente un módulo recursivo que reciba la lista generada en a) y devuelva el mínimo
valor de la lista.
e. Implemente un módulo recursivo que reciba la lista generada en a) y un valor y devuelva
verdadero si dicho valor se encuentra en la lista o falso en caso contrario.}


program ejer2prac2;
type


    lista =^nodo;

    nodo = array 
        elem:integer;
        sig:lista;
    end;

{a. Implemente un módulo recursivo que genere y retorne una lista de números enteros
“random” en el rango 100-200. Finalizar con el número 100.}

procedure ListaRecursiva(var l:lista);
var
num:integer;
begin
    leerNum(num);
    if (num <> 100) then begin
        cargarLista(l,num);
        ListaRecursiva(l);
    end;
end;

procedure leerNum(var num:integer);
var rango:integer;
begin
    rango := 200-100;
    num := random (rango)
end;


procedure cargarLista(var  l:lista, num:integer);
var
nuevo:lista;
begin
    new(nuevo); nuevo^.elem:=num; nuevo^.sig:= nil;
    if(l = nil) then l:= nuevo;
    else
    begin
        nuevo^.sig:=l
        l:=nuevo;
    end;
end;


{b. Un módulo recursivo que reciba la lista generada en a) e imprima los valores de la lista en el
mismo orden que están almacenados.}

procedure ImprimirListaEnOrden (l:lista);
var
begin
    if (l <> nil) then 
    writeln('el valor es :', l^.elem);
    l:= l^.sig;
    ImprimirListaEnOrden(l);
    end;
end;

{c. Implemente un módulo recursivo que reciba la lista generada en a) e imprima los valores de
la lista en orden inverso al que están almacenados.}


procedure ImprimirListaInverso (l:lista);
var
begin
    if (l <> nil) then 
    ImprimirListaInverso(l^.sig);
    writeln('el valor es :', l^.elem);
    end;
end;

{d. Implemente un módulo recursivo que reciba la lista generada en a) y devuelva el mínimo
valor de la lista.}

procedure MinimoLista(l:lista; var min:integer);
begin
    if(l<>nil) then begin
        if(l^.elem > min) then 
            min:=l^.elem;
        l:=l^.sig;
        MinimoLista(l,min);
    end;
end;

{e. Implemente un módulo recursivo que reciba la lista generada en a) y un valor y devuelva
verdadero si dicho valor se encuentra en la lista o falso en caso contrario.}

function Listaboolean(l:lista; valor:integer):boolean;
begin
    Listaboolean:=false;
    if (l<>nil) then begin
        if(l^.elem = valor) then
            Listaboolean:=true;
        l:=l^.sig;
        Listaboolean(l,valor);
    end;
end;

var

l:lista;
valor,min:integer;

begin
    min:=999;
    ListaRecursiva(l);
    ImprimirListaEnOrden(l);
    ImprimirListaInverso(l);
    MinimoLista(l,min);
    writeln('ingresar valor a buscar:');
    readln(valor);
    writeln('la condicion con el valor ingresado es :' Listaboolean(l,valor))
end.
