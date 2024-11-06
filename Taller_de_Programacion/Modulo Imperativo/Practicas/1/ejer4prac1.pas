{4.- Una librería requiere el procesamiento de la información de sus productos. De cada
producto se conoce el código del producto, código de rubro (del 1 al 8) y precio.
Implementar un programa que invoque a módulos para cada uno de los siguientes puntos:
a. Lea los datos de los productos y los almacene ordenados por código de producto y
agrupados por rubro, en una estructura de datos adecuada. El ingreso de los productos finaliza
cuando se lee el precio 0.
b. Una vez almacenados, muestre los códigos de los productos pertenecientes a cada rubro.
c. Genere un vector (de a lo sumo 30 elementos) con los productos del rubro 3. Considerar que
puede haber más o menos de 30 productos del rubro 3. Si la cantidad de productos del rubro 3
es mayor a 30, almacenar los primeros 30 que están en la lista e ignore el resto.
d. Ordene, por precio, los elementos del vector generado en c) utilizando alguno de los dos
métodos vistos en la teoría.
e. Muestre los precios del vector resultante del punto d).
f. Calcule el promedio de los precios del vector resultante del punto d).}
program ejer4prac1;
type

    subrangoRubro = [1..8];

    productos:= record
        codigoProducto:integer;
        codigoRubro:subrangoRubro;
        precio: real;
    end;
    
    lista = ^nodo;

    nodo = record
        elem:productos;
        sig:lista;
    end;

    vectorProductos = array [subrangoRubro] of lista;

    vectorRubro3 = array [1..30] of lista;

{a. Lea los datos de los productos y los almacene ordenados por código de producto y
agrupados por rubro, en una estructura de datos adecuada. El ingreso de los productos finaliza
cuando se lee el precio 0.}
    
procedure CargarLista(var vectorLista:vectorProductos);
var
elem:pelicula;
begin
    inicializar(vectorLista);
    LeerElemento(elem);
    while (elem.precio <> 0) do begin
        insertarLista(vectorLista[elem.codigoRubro], elem);
        LeerElemento(elem);
    end;
end;


procedure inicializar(var v:vectorProductos);
var
i:integer;
begin
    for i:=1 to 8 do 
        v[i]:= nil;
end;

procedure LeerElemento(var e:pelicula);
begin   
    writeln('ingresar codigo de producto: ');
    readln(e.codigoProducto);
    writeln('ingresar codigo de rubro');
    readln(e.codigoRubro);
    writeln('ingresar preci0 ');
    readln(e.precio);
end;


procedure insertarLista(var l:lista ; e:elem);
Var
 actual,anterior,nuevo:lista;
Begin
  new (nuevo); nuevo^.elem:= e; nuevo^.sig:=nil;
  if (l = nil) then     l:= nuevo
  
  else begin
    actual:= ;; ant:=l;
    while (actual <> nil) and (actual^.e.codigoProducto < nuevo^.elem.codigoProducto) do   
     begin
      anterior:=actual;
      actual:= actual^.sig;
     end;
    if (actual = l) then 
    begin
    nuevo^.sig:= l;   l:= nuevo;
    end
    else begin
    anterior^.sig:= nuevo;   nuevo^.sig:= actual;
    end;
End;
    
  end;

{b. Una vez almacenados, muestre los códigos de los productos pertenecientes a cada rubro.}

procedure CodigosxRubro(vectorLista: vectorProductos);
var
i:integer;
begin
    for i:=1 to 8 do begin
        recorrerVectorLista(vectorLista[i], i);
    end;
end;

procedure recorrerVectorLista (l:lista, pos:integer;);
begin
    writeln('los codigos que pertenecen al rubro ', pos , 'son');
    while (l <> nil) do begin 
        writeln('codigo:', l^.elem.codigoProducto);
        l:=l^.sig;
    end;
    writeln('ya se vieron todos los codios de preducto del rubro:', pos);
end;

{c. Genere un vector (de a lo sumo 30 elementos) con los productos del rubro 3. Considerar que
puede haber más o menos de 30 productos del rubro 3. Si la cantidad de productos del rubro 3
es mayor a 30, almacenar los primeros 30 que están en la lista e ignore el resto.}


procedure rubro3 ( var v:vectorRubro3; vectorLista:vectorProductos; var diml :integer);
begin
    cargarElVector(vectorLista[3], v);
end;

procedure cargarElVector(l:lista; var v:vectorRubro3; var diml :integer;);
var
begin
    while (l<>nil) and (diml < =30 ) do begin
            diml : = diml +1;
            v[diml]:=lista;
        l:= l^.sig;
    end;
end;

{d. Ordene, por precio, los elementos del vector generado en c) utilizando alguno de los dos
métodos vistos en la teoría.}

procedure inserción( var v:vectorRubro3; diml:integer);
var
    i,j:integer; actual:expensas; 

begin
    for i:=2 to diml do begin {recorre todo el vector.}
        actual:= v[i].precio;  
        j:= i - 1;
        while (j > 0 ) and (v[j].precio > actual) do begin  
            v[j+1]:= v[j]; {remplaza la posicion}
            j:= j - 1;
        end;
        v[j+1]:= actual;
    end;
end;

{e. Muestre los precios del vector resultante del punto d}

procedure MostrarResultados(v:vectorRubro3, diml : integer, var sumatotal:integer;);
var
begin
    for i:= 1 to diml do begin
        writeln('el precio de la posicion ', i, 'es : ', v[i]);
        sumatotal:= sumatotal + v[i];
    end;
end;

function promedio(suma:integer; diml:integer):real;
var 
resul:real;
begin
    resul:= suma/diml;
    promedio:=resul;
end;



var
vectorLista:vectorProductos;
v3:vectorRubro3;
diml, sumatotal : integer;
begin
diml:=0;
sumatotal:=0;
    CargarLista(vectorLista);
    CodigosxRubro(vectorLista);
    rubro3(v3,vectorLista,diml);
    inserción(v3,diml);
    MostrarResultados(v3,diml,sumatotal);
    writeln(promedio(sumatotal,diml));
end.






