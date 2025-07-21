program ejer17_prac2;

const
    VALOR_ALTO = 999;
    CANT_DETALLES = 10;

type

    maestro = record
        code:integer;
        nombre:string;
        descripcion:string;
        modelo:string;
        marca:string;
        stockActual:integer;
    end;

    detalle = record
        code: integer;
        precio:real;
        fechaVenta:integer;
    end;

    fileMaestro = file of maestro;

    fileDetalle = file of detalle;

    arrayDetalles = array [1.. CANT_DETALLES] of fileDetalle;
    arrayRegs = array [1.. CANT_DETALLES] of detalle;\


procedure leer(var arch:fileDetalle; var dato:detalle);
begin
    if(not eof(arch))then
        read(arch, dato);
    else
        dato.code:= VALOR_ALTO;
end;

procedure minimo(var vd: arrayDetalles; var vr:arrayRegs; var min:detalle);
var
    posMin,i : integer;
begin
    min.code:= VALOR_ALTO;
    for i:=1 to CANT_DETALLES do begin
        if(vr[i].code < min.code) then begin
            min:=vr[i];
            posMin:=i;
        end;
    end;
    leer(vd[posMin], vr[posMin]);
end;

procedure actualizar(var mae: fileMaestro);
var
    vd:arrayDetalles;
    vr:arrayRegs;
    i:integer;
    min:detalle;
    regm:maestro;
    codeMasVendida:integer;
    stockMin:integer;

begin

    //asigno, abro y leo los primeros 10 detalles.
    for i:=1 to CANT_DETALLES do begin
        assing(vd[i],  'detalle', i);
        reset(vd[i]);
        leer(vd[i], vr[i]);
    end;

    //abro maestro;
    reset(mae);
    read(mae,regm)

    //calculo el minimo de esos 10
    minimo(vd, vr,min);

    while(min.code<> VALOR_ALTO) do begin
        
        while(regm.code <> min.code) do
            read(mae,regm);

        while(regm.code = min.code) do begin
            regm.stockActual:=regm.stockActual - 1;
            minimo(vd, vr,min);
        end;

        //una vez que cambio mi moto  escribio
        seek(mae, filepos(mae)-1);
        write(mae, regm);

        if (not eof(mea)) then
            read(mae,regm);
    end;

    //cuando termine de actualizar el maestro informo:
    seek(mae, 9);
    
    stockMin:=9999;

    while not eof(mae) do begin
        read(mae, regm);
        if(regm.stockActual < stockMin) then begin
            stockMin:= regm.stockActual;
            codeMasVendida:=regm.code;
        end;
    end;
    writeln('la moto mas vendida fue:', codeMasVendida);
    // aca se que tendria que informas marca, modelo etc. lo hago para ahorrar code.
end;


var
    mae:fileMaestro;

begin
    assign(mae, 'maestro');
    actualizar(mae);

end;
