{3.- Netflix ha publicado la lista de películas que estarán disponibles durante el mes de
diciembre de 2022. De cada película se conoce: código de película, código de género (1: acción,
2: aventura, 3: drama, 4: suspenso, 5: comedia, 6: bélico, 7: documental y 8: terror) y puntaje
promedio otorgado por las críticas.

Implementar un programa que invoque a módulos para cada uno de los siguientes puntos
a. Lea los datos de películas, los almacene por orden de llegada y agrupados por código de
género, y retorne en una estructura de datos adecuada. La lectura finaliza cuando se lee el
código de la película -1.
b. Genere y retorne en un vector, para cada género, el código de película con mayor puntaje
obtenido entre todas las críticas, a partir de la estructura generada en a)..
c. Ordene los elementos del vector generado en b) por puntaje utilizando alguno de los dos
métodos vistos en la teoría.
d. Muestre el código de película con mayor puntaje y el código de película con menor puntaje,
del vector obtenido en el punto c).}

program ejer3prac1;

type

    subrangoPeliculas = [1..8];

    pelicula = record 
        codigoPelicula:integer;
        codigoGenero:subrangoPeliculas;
        puntajePromedio:real;
    end;

    lista = ^nodo;

    nodo = record
        elem:= pelicula;
        sig:=lista;
    end;

    VectoPeliculas = array [subrangoPeliculas] of lista;

    VectorUp = array [subrangoPeliculas] of lista;

    vectorcontador = array [subrangoPeliculas] of integer;




{a. Lea los datos de películas, los almacene por orden de llegada y agrupados por código de
género, y retorne en una estructura de datos adecuada. La lectura finaliza cuando se lee el
código de la película -1.}


procedure(var v:VectoPeliculas);
var
i: integer;
begin
    for i:=1 to 8 do 
        v[i]:= nil;
end;
procedure(var v:VectorUp);
var
i: integer;
begin
    for i:=1 to 8 do 
        v[i]:= nil;
end;

procedure CargarLista (var v:VectoPeliculas; var vup:VectorUp);
var
elemento:pelicula;
begin
    inicializar(v);
    inicializar(vup);
    LeerElemento(elemento);
    while (elemento.codigoPelicula <> -1) do begin
        agregarAlFinal(v[elemento.codigoGenero], vup[elemento.codigoGenero],elemento);
        LeerElemento(elemento);
    end;
end;

procedure LeerElemento(var e:pelicula);
begin
    writeln('Ingrese codigo de pelicula');
    readln(e.codigoPelicula);
    writeln('Ingrese codigo de genero');
    readln(e.codigoGenero);
    writeln('Ingrese puntaje promedio de criticas');
    readln(e.puntajePromedio); 
end;


procedure agregarAlFinal(var pI:lista; var pU:lista; e: elemento);
var
nuevo,aux:lista;
begin
    new(nuevo); nuevo^.elem:= elemento; nuevo^.sig:=nil;
    if (pI =nil) then begin
        pI:= nuevo;
        pU:=nuevo;
    end
    else begin
        pU^.sig:= nuevo;
        pU:=nuevo;   {actualizo el siguiente del ultimo nodo y al ultimo nodo}
    end;
end;


{b. Genere y retorne en un vector, para cada género, el código de película con mayor puntaje
obtenido entre todas las críticas, a partir de la estructura generada en a)..}

function maxCodigos(l:lista):integer;
var
    maximo: integer;
begin
maximo:=-999;
    
end;

procedure Maximos(var vc:vectorcontador; vectorMaximos:VectoPeliculas);
var
i:integer;
begin
    vectoren0(v);
    for i:=1 to 8 do begin
        vc[i] := maxCodigos(vectorMaximos[i]);

    end;
    
end;


procedure vectoren0(var v:vectorcontador);
var
i:integer;
begin
    for i:=1 to 8 do    
        v[i]:= 0;
end;