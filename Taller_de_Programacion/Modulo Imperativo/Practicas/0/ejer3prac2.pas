{3.- Implementar un programa que invoque a los siguientes módulos.
a. Un módulo recursivo que retorne un vector de 20 números enteros “random” mayores a 300
y menores a 1550 (incluidos ambos).
b. Un módulo que reciba el vector generado en a) y lo retorne ordenado. (Utilizar lo realizado
en la práctica anterior)
c. Un módulo que realice una búsqueda dicotómica en el vector, utilizando el siguiente
encabezado:
Procedure busquedaDicotomica (v: vector; ini,fin: indice; dato:integer; var pos: indice);
Nota: El parámetro “pos” debe retornar la posición del dato o -1 si el dato no se encuentra
en el vector.}


program ejer3prac2;
const
    dimf:20;
type
    vector = array [1..dimf] of integer;


{a. Un módulo recursivo que retorne un vector de 20 números enteros “random” mayores a 300
y menores a 1550 (incluidos ambos).}

procedure CargarVector(var v:vector; var diml:integer);
var
num:integer;
begin
    obtenerNumeroRandom(num);
    if (diml>=dimf) then begin
        diml := diml + 1;
        v[diml]:=num;
        CargarVector(v,diml);
    end;
end;


procedure obtenerNumeroRandom(var num);
begin
    num:= random(300 .. 1550);
end;

{b. Un módulo que reciba el vector generado en a) y lo retorne ordenado. (Utilizar lo realizado
en la práctica anterior)}

procedure OrdenarVector(var v:vector; diml:integer;);
var i,j,actual:integer;
begin
    for i:= 2 to diml do begin {el i empieza en la pos 2}
        actual:=v[i]; {lo caul hace que el actual sea igual a lo que vale mi vector en la posicion 2 o la siguiente a j de manera general}
        j:=i-1{j vale 1 menos que i}
        while(j>0) and (v[j] > actual) do  {mientras j mayor a 0 y lo que vale la pos 1 en este caso de mas grande que lo q hay en la pos 2}
        begin
                v[j+1]:=v[j]; {si pasa lo que esta en la pos 2 es igual a lo que esta en la pos 1}
                j:= j-1;{y a j se le resta 1 en este caso quedria en 0}
        end;
        v[j+1]:= actual; {para que aca 0 + 1 y el actual me quede en uno y ahi se guarde el actual}
    end;  
end;

{c. Un módulo que realice una búsqueda dicotómica en el vector, utilizando el siguiente
encabezado:
Procedure busquedaDicotomica (v: vector; ini,fin: indice; dato:integer; var pos: indice);
Nota: El parámetro “pos” debe retornar la posición del dato o -1 si el dato no se encuentra
en el vector.}

Procedure busquedaDicotomica (v: vector; ini,fin: indice; dato:integer; var pos: indice);
var
medio:integer;
begin
    medio:= (ini + ult) div 2;
    while(ini<= ult) and (dato<>v[medio]) do begin
        if(dato < v[medio]) then 
            ult := medio -1;
        else
        pri:= medio +1;
        medio:= (pri + ult) div 2
    end;
    if(pri<= ult) and (dato = v[medio]) then pos:=medio;
    else
    pos:=-1;
    end;
end;
