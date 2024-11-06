program parcial2;
type


    subrangoca = 1 .. 5;

    datafechas = record
        dia:integer;
        mes:integer;
        anio:integer;
    end;

    participante = record
        dni:integer;
        nombre:string;
        apellido:string;
        categoria: subrangoca;
        fecha:datafechas;
    end;

    v = array [1.. 5000] of participante;


    
    vectorContador = array [subrangoca] of integer;

    lista = ^nodo;

    nodo= record;
        elem:participante;
        sig:lista;
    end;

    procedure cargarVectorParticipantes(var v:vector);
    begin
        {se dispone}
    end;

    procedure vectorContadora0(var vc:vectorContador);
    var
    i:integer;
    begin
        for i:=1 to 5 then 
            vc[i]:=0;
    end;

    procedure Llenarvc(var vc:vectorContador; v:vector);
    begin
        for i:=1 to 5000 then
            vc[v.categoria]:= vc[v.categoria] + 1;
    end;
    
    procedure agregarAdelante (var l:lista; vc:vectorContador; v:vector);
    var
    nuevo:lista;
    i:integer;
    begin
        for i:=1 to 5000 then begin
            if (vc[v[i].categoria]>= 50) then begin
                new(nuevo); nuevo^.elem:=v[i]; nuevo^.sig=nil;
                else
                begin
                    nuevo^.sig:=nil;
                    l:=nuevo;
                end;
            end;
        end;

    end;


var
l:lista;
v:vector
vc:vectorContador
begin
    l=nil;
    cargarVectorParticipantes(v);
    vectorContadora0(vc);
    Llenarvc(vc,v);
    agregarAdelante(l,v);
end;