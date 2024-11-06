{1. El administrador de un edificio de oficinas tiene la información del pago de las expensas
de dichas oficinas. Implementar un programa con:
a) Un módulo que retorne un vector, sin orden, con a lo sumo las 300 oficinas que
administra. Se deben cargar, para cada oficina, el código de identificación, DNI del
propietario y valor de la expensa. La lectura finaliza cuando llega el código de
identificación 0
b) Un módulo que reciba el vector retornado en a) y retorne dicho vector ordenado por
código de identificación de la oficina. Ordenar el vector aplicando uno de los métodos
vistos en la cursada
c) Un módulo que realice una búsqueda dicotómica. Este módulo debe recibir el vector
generado en b) y un código de identificación de oficina. En el caso de encontrarlo, debe
retornar la posición del vector donde se encuentra y en caso contrario debe retornar 0.
Luego el programa debe informar el DNI del propietario o un cartel indicando que no
se encontró la oficina
d) Un módulo recursivo que retorne el monto total de las expensas}

program ejer1prac5;
const 
dimf = 300;
type

    oficina = record
        codeIden:integer;
        dni:integer;
        expensa:real;
    end;

    

    vector = array[1..dimf] of oficina;



    procedure cargarVector(var v:vector; var diml:integer);

        procedure leerElemento (var elem:oficina);
        begin
            writeln('ingrese codigo de idem:');
            readln(elem.codeIden);
            writeln('ingrese DNI:');
            readln(elem.dni);
            writeln('ingrese costo de expensas:');
            readln(elem.expensa);         
        end;

    var
    elem:oficina;
    begin
        leerElemento(elem);
        while (diml < dimf) and (elem.codeIden <> 0) do begin
            diml:= diml +1;
            v[diml]:= elem;
            leerElemento(elem);
        end;
    end;

    procedure ordenarVector(var v:vector; diml:integer);
    var
    i,j:integer;
    actual:oficina;
    begin
        for i:=2 to diml do begin   
            actual:=v[i]; {actual es iguazl a lo que hay en v es la pos 2}
            j:=i-1;{ situa a j en la pos 1 del vector}
            while(j>0) and (v[j].codeIden > actual.codeIden) do {mientras j no sea 0 (para poner en la primera pos) y mientras el valor anterior sea mas chico que el que le sigue}
            begin
                v[j+1]:=v[j];{sumo una pos a j}
                j:=j-1;
            end;
            v[j+1]:= actual;
        end;
    end;


    procedure busquedaDico(v:vector; ini,fin,code:integer;  var dniEncontrado:integer;  var Poscode:integer);
    var
    medio:integer;
    begin
        medio:=(ini + fin) div 2;
        if(v[medio].codeIden <> code) and (ini < fin) then begin
            if(v[medio].codeIden > code) then
                busquedaDico(v,ini,(medio - 1),code, dniEncontrado, Poscode)
            else
                busquedaDico(v,(medio + 1), fin, code, dniEncontrado, Poscode);
        end
        else if(v[medio].codeIden = code) then begin
                Poscode:= medio;
                dniEncontrado:=v[medio].dni;
            end
            else
                Poscode:=0;
    end;


    function montoTotal(v:vector; diml:integer; var monto:real; var pos:integer):real;
    begin
        if(pos < diml) then begin
            pos:=pos +1;
            monto:= monto + v[pos].expensa;
            montoTotal:=montoTotal(v,diml,monto,pos);
        end;
        montoTotal:=monto;
    end;
var
v:vector;
diml:integer;
code:integer;
dniEncontrado:integer;
Poscode:integer;
monto:real;
pos:integer;
begin
    pos:=0;
    monto:=0;
    diml:=0;
    Poscode:=0;
    dniEncontrado:=0;
    cargarVector(v,diml);
    ordenarVector(v,diml);
    writeln('ingresa un codigo de indentificacion para buscarlos: ');
    readln(code);
    busquedaDico(v,1,diml,code,dniEncontrado,Poscode);
    writeln('la pos es :', Poscode);
    if(Poscode <> 0) then
        writeln('el dni del cliente es', dniEncontrado)
    else
        writeln('no se encontro la oficina');
   writeln('el monto total fue de:', montoTotal(v,diml,monto,pos):2:2);
end.