{2.- El administrador de un edificio de oficinas cuenta, en papel, con la información del pago de
las expensas de dichas oficinas.
Implementar un programa que invoque a módulos para cada uno de los siguientes puntos:
a. Genere un vector, sin orden, con a lo sumo las 300 oficinas que administra. De cada oficina
se ingresa el código de identificación, DNI del propietario y valor de la expensa. La lectura
finaliza cuando se ingresa el código de identificación -1, el cual no se procesa.
b. Ordene el vector, aplicando el método de inserción, por código de identificación de la
oficina.
c. Ordene el vector aplicando el método de selección, por código de identificación de la oficina}


program ejer2prac1;

const

dimf : 300;


type

expensas = record
    codeIden:integer;
    dni:integer;
    valorExpensas:real;
end;

vector  =  array [1..dimf] of expensas;


procedure CargarVector(var v:vector; var diml:integer);
var
begin
    repeat

        diml:= diml + 1;

        write('ingrese codigo de indentificacion: ');
        read(v[diml].codeIden);
        write('ingrese el dni del propietario: ');
        read(v[diml].dni);
        write('valor de las expensas: ');
        read(v[diml].valorExpensas);

        
    until (v.codeIden <> -1 ); {finaliza cuando se ingresa el código de identificación -1, el cual no se procesa.}
end;

{b. Ordene el vector, aplicando el método de inserción, por código de identificación de la
oficina.}

procedure inserción( var v:vector; diml:integer);
var
    i,j:integer; actual:expensas; 

begin
    for i:=2 to diml do begin {recorre todo el vector.}
        actual:= v[i].codeIden;  
        j:= i - 1;
        while (j > 0 ) and (v[j].codeIden > actual) do begin  
            v[j+1]:= v[j]; {remplaza la posicion}
            j:= j - 1;
        end;
        v[j+1]:= actual;
    end;
end;

{b. Ordene el vector, aplicando el método de inserción, por código de identificación de la
oficina.}

procedure selección (var :vector; diml:integer);
var
i,j :integer; pos:iinteger ; actual:expensas;
begin
    for i:= 1 to (diml -1) do begin 
        pos:=i;
        for j:=i +1 to diml do 
            if v[j].codeIden < v[pos].codeIden then pos:=j;
        
        {intercambio.}
         item := v[pos];
         v[pos] := v[i];
         v[i] := item;
    end;
end;


var
v:vector;
diml:integer;
begin
    diml:=0;
    CargarVector(v,diml);
    inserción(v,diml);
    selección(v,diml);    
end.


