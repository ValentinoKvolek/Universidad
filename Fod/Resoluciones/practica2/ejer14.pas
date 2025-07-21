program ejer14_prac2;
const
 VALOR_ALTO = 'ZZZZ'

type

    maestro = record
        destino:string;
        fecha:string;
        hora:real;
        cantAsientoDisponibles:integer;
    end;

    detalle = record
        destino :string;
        fecha:string;
        horaSalida:real;
        cantAsientoComprado:integer;
    end;

    //ordenados por destino mas fehca y hora de salida.

    fileMaestro = file of maestro;
    fileDetalle = file of detalle;

procedure leer(var arch : fileDetalle; var dato : detalle);
begin
    if(not eof(arch)) then 
        read(arch, dato)
    else
        dato.destino:= VALOR_ALTO;

procedure minimo(var det1, det2: fileDetalle; var regd1, regd2, min: detalle);
begin
  if (regd1.destino < regd2.destino) or
     ((regd1.destino = regd2.destino) and (regd1.fecha < regd2.fecha)) or
     ((regd1.destino = regd2.destino) and (regd1.fecha = regd2.fecha) and (regd1.horaSalida < regd2.horaSalida)) then
  begin
    min := regd1;
    leer(det1, regd1);
  end
  else
  begin
    min := regd2;
    leer(det2, regd2);
  end;
end;


procedure listar(var mae:fileMaestro; var regm: maestro );
var
    input:integer;
begin
    writeln('ingrese cantidad de asientos disponible minima para listar.');
    readln(input);
    reser(mae);
    while( not eof(mae)) do begin
        read(mae, regm);
        if(regm.cantAsientoDisponibles > input) then begin
            writeln('destino:', regm.destino, ' fecha:', regm.fecha, ' hora de salida:', regm.hora);
    end;
    close(mae);
end;


var
    mae:fileMaestro;
    regm :maestro;
    det1,det2:fileDetalle;
    regd1,regd2, min: detalle;
begin

    assing(mae, 'maestro');
    assing(det1, 'detalle1');
    assign(det2, ' detalle2');

    reset(mae);
    reset(det1);
    reset(det2);

    leer(det1, regd1);
    leer(det2,regd2);
    minimo(det1,det2, regd1,regd2, min);
    read(mae, regm);

    while min.destino <> VALOR_ALTO do
    begin
        read(mae, regm);
        while (regm.destino <> min.destino) or (regm.fecha <> min.fecha) or (regm.hora <> min.horaSalida) do
            read(mae, regm);

        while (regm.destino = min.destino) and (regm.fecha = min.fecha) and (regm.hora = min.horaSalida) do begin
            regm.cantAsientoDisponibles := regm.cantAsientoDisponibles - min.cantAsientoComprado;
            minimo(det1, det2, regd1, regd2, min);
        end;

        seek(mae, filepos(mae) - 1);
        write(mae, regm);
    end;

    close(mae);
    close(det1);
    close(det2);

    listar(mae, regm);
end.