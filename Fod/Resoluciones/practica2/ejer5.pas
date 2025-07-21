program ejer5prac2;

uses
    SysUtils;

const
    CANT_MAQUINAS = 5;
    VALOR_ALTO = 999;

type
    detalle = record
        cod_usuario: integer;
        fecha: string[10];  
        tiempo_sesion: real;
    end;

    maestro = record
        cod_usuario: integer;
        fecha: string[10];  
        tiempo_total_de_sesiones_abiertas: real;
    end;

    fileMaestro = file of maestro;
    fileDetalle = file of detalle;
    detalles = array[1..CANT_MAQUINAS] of fileDetalle;
    regDetalles = array[1..CANT_MAQUINAS] of detalle;

procedure leer(var arch: fileDetalle; var dato: detalle);
begin
    if not eof(arch) then
        read(arch, dato)
    else
        dato.cod_usuario := VALOR_ALTO;
end;

procedure minimo(var dets: detalles; var regs: regDetalles; var min: detalle);
var
    i, posMin: integer;
begin
    min.cod_usuario := VALOR_ALTO;
    posMin := 1;
    for i := 1 to CANT_MAQUINAS do
        if (regs[i].cod_usuario < min.cod_usuario) or 
           ((regs[i].cod_usuario = min.cod_usuario) and (regs[i].fecha < min.fecha)) then begin
            min := regs[i];
            posMin := i;
        end;
    leer(dets[posMin], regs[posMin]);
end;

procedure CrearMaestro(var vd: detalles);
var
    i: integer;
    vrd: regDetalles;
    mae: fileMaestro;
    regm: maestro;
    min: detalle;
    actual_cod: integer;
    actual_fecha: string[10];
    total: real;
begin
    assign(mae, 'maestro');  
    rewrite(mae);

    for i := 1 to CANT_MAQUINAS do begin
        assign(vd[i], 'detalle' + IntToStr(i));
        if FileExists('detalle' + IntToStr(i)) then begin
            reset(vd[i]);
            leer(vd[i], vrd[i]);
            writeln('detalle', i, ': cod=', vrd[i].cod_usuario, 
                    ', fecha=', vrd[i].fecha, ', tiempo=', vrd[i].tiempo_sesion:0:2);
        end else
            writeln('Error: detalle', i, ' no existe');
    end;

    minimo(vd, vrd, min);

    while min.cod_usuario <> VALOR_ALTO do begin
        actual_cod := min.cod_usuario;
        actual_fecha := min.fecha;
        total := 0;
        while (min.cod_usuario = actual_cod) and (min.fecha = actual_fecha) do begin
            total := total + min.tiempo_sesion;
            minimo(vd, vrd, min);
        end;
        regm.cod_usuario := actual_cod;
        regm.fecha := actual_fecha;
        regm.tiempo_total_de_sesiones_abiertas := total;
        write(mae, regm);

    end;

    close(mae);

    for i := 1 to CANT_MAQUINAS do
        if FileExists('detalle' + IntToStr(i)) then
            close(vd[i]);


    reset(mae);
    while not eof(mae) do begin
        read(mae, regm);
        writeln('usuario: ', regm.cod_usuario, ' fecha: ', regm.fecha, 
                ' total horas: ', regm.tiempo_total_de_sesiones_abiertas:0:2);
    end;
    close(mae);
end;

var
    vd: detalles;

begin
    CrearMaestro(vd);
end.