program ejer16_prac2;

const
    VALOR_ALTO = 9999;

type

    maestro = record
        fecha = integer;
        codeSemanario:integer;
        descripcion:string;
        precio:real;
        totalEjem:integer;
        totalEjemVendidos:integer;
    end;

    detalle = record    
        
        fecha:integer;
        codeSemanario:integer;
        cantEjemVendidos:integer;
    end;

    fileMaestro = file of maestro;
    fileDetalle = file of detalle;

    arrayDetalles = array [1 .. 100] of fileDetalle;
    arrayRegs = array [1..100] of detalle;

procedure leer(var arch: fileDetalle; var dato:detalle);
begin
    if(not eof(arch)) then
        read(arch, dato)
    else
        dato.fecha:= VALOR_ALTO
end;

procedure minimo (var vd: arrayDetalles; var vr: arrayRegs; var min : detalle);
var
    posMin:integer;
begin
    posMin:=1;
    min.fecha:= VALOR_ALTO;
    for i:=1 to 100 do begin
        if (vr[i].fecha < min.fecha) or (vr[i].fecha = min.fecha) and(vr[i].codeSemanario < min.codeSemanario) then begin
            min:= vr[i];
            posMin:=i;
        end;
    end;
    leer(vd[posMin], vr[posMin]);
end;

procedure actualizar (var mae: fileMaestro; var vd:arrayDetalles);
var
    min:detalle;
    regm:maestro;
    vr:arrayRegs;
    i:integer;
begin
    for i:=1 to 100 do begin
        reset(vd[i]);
        leer(vd[i], vr[i]);
    end;
    reser(mae);
    read(mae, regm);
    minimo(vd, vr, min);
    while (min.fecha <> VALOR_ALTO) do begin

        
        while (regm.fecha <> min.fecha) or (regm.codeSemanario <> min.codeSemanario) do
            read(mae, regm);  

        
        while (regm.fecha = min.fecha) and (regm.codeSemanario = min.codeSemanario) do begin
            regm.totalEjemVendidos := regm.totalEjemVendidos + min.cantEjemVendidos;
            regm.totalEjem := regm.totalEjem - min.cantEjemVendidos;
            minimo(vd, vr, min);
        end;

        seek(mae, filepos(mae) - 1);
        write(mae, regm);

        if not eof(mae) then
            read(mae, regm);
    end;

    close(mae);


    for i:=1 to 100 do 
        close(vd[i]);
end; 

var 

    mae:fileMaestro;
    vd:arrayDetalles;

    max:integer;
    codeMax:integer;
    fechaMax:integer;
    min:integer;
    codeMin:integer;
    fechaMin:integer;


begin

    assing(mae, 'maestro');
    for i:=1 to 100 do begin
        assing(vd[i], 'detalle', i);
    end;
    actualizar(mae, vd );

    reset(mae);

    max:=-9999;
    codeMax:=0;
    fechaMax:=0;
    min:=--9999;
    codeMin:=0;
    fechaMin:=0;
    while not eof(mae) do begin
        read(mae, regm);
        if(regm.cantEjemVendidos > max) then begin
            max:=regm.cantEjemVendidos;
            codeMax:=regm.codeSemanario;
            fechaMax:=regm.fecha;
        end;
        if(regm.cantEjemVendidos < min) then begin
            min:=regm.cantEjemVendidos;
            codeMin:=regm.codeSemanario;
            fechaMin:=regm.fecha;
        end;
    end;

    write(codeMin,fechaMin, codeMax, fechaMax);
end.
