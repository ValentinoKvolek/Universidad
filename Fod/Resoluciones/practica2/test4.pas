program GenerarArchivosPruebaEjer4;

uses
    SysUtils; // Agregamos esta unidad para IntToStr

const
    CANT_SUCURSALES = 30;

type
    maestro = record
        codeProducto: integer;
        nombre: string;
        descripcion: string; // Sin tilde para evitar problemas de codificación
        stockDiponible: integer;
        stockMinimo: integer;
        precio: real;
    end;

    detalle = record
        codeProducto: integer;
        cantVendida: integer;
    end;

    fileMaestro = file of maestro;
    fileDetalle = file of detalle;
    detalles = array[1..CANT_SUCURSALES] of fileDetalle;

procedure CrearMaestro(var mae: fileMaestro);
var
    regm: maestro;
    i: integer;
begin
    assign(mae, 'maestro4');
    rewrite(mae);

    for i := 1 to 10 do begin
        regm.codeProducto := i * 100; // 100, 200, ..., 1000
        regm.nombre := 'Producto' + IntToStr(i);
        regm.descripcion := 'Descripcion del producto ' + IntToStr(i); // Sin tilde
        regm.stockDiponible := 50 + random(51); // Entre 50 y 100
        regm.stockMinimo := 10 + random(21); // Entre 10 y 30
        regm.precio := 10.0 + random(91); // Entre 10.0 y 100.0
        write(mae, regm);
    end;

    close(mae);
    writeln('Archivo maestro "maestro4" creado con éxito.');
end;

procedure CrearDetalles(var dets: detalles);
var
    regd: detalle;
    i, j, numRegistros, baseCode: integer;
    temp: array[1..5] of detalle;
    k: integer;
begin
    for i := 1 to CANT_SUCURSALES do begin
        assign(dets[i], 'detalle' + IntToStr(i));
        rewrite(dets[i]);

        numRegistros := 1 + random(5);
        for j := 1 to numRegistros do begin
            baseCode := (1 + random(10)) * 100;
            temp[j].codeProducto := baseCode;
            temp[j].cantVendida := 1 + random(10);
        end;

        for j := 1 to numRegistros - 1 do begin
            for k := 1 to numRegistros - j do begin
                if temp[k].codeProducto > temp[k + 1].codeProducto then begin
                    regd := temp[k];
                    temp[k] := temp[k + 1];
                    temp[k + 1] := regd;
                end;
            end;
        end;

        for j := 1 to numRegistros do
            write(dets[i], temp[j]);

        close(dets[i]);
    end;
    writeln('30 archivos detalle creados con éxito (detalle1 a detalle30).');
end;

var
    mae: fileMaestro;
    dets: detalles;

begin
    randomize;
    CrearMaestro(mae);
    CrearDetalles(dets);
    writeln('Generación de archivos completada.');
end.