procedure insercion(var v:vector; diml:integer);
var
    i,j,actual: integer;
begin

    for i:=2 to diml then begin
        actual:= v[i] {me guardo la posicion de "adelante" para ir comparando}
        j:= i-1
        while (j>0) and (v[j]> actual) do begin
            v[j+1]:= v[j]
            j:=j-1
        end
        v[j+1]:=actual;
    end;