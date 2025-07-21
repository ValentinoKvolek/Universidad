(*
4. Se cuenta con un archivo de productos de una cadena de venta de alimentos congelados.
De cada producto se almacena: código del producto, nombre, descripción, stock disponible,
stock mínimo y precio del producto.
Se recibe diariamente un archivo detalle de cada una de las 30 sucursales de la cadena. Se
debe realizar el procedimiento que recibe los 30 detalles y actualiza el stock del archivo
maestro. La información que se recibe en los detalles es: código de producto y cantidad
vendida. Además, se deberá informar en un archivo de texto: nombre de producto,
descripción, stock disponible y precio de aquellos productos que tengan stock disponible por
debajo del stock mínimo. Pensar alternativas sobre realizar el informe en el mismo
procedimiento de actualización, o realizarlo en un procedimiento separado (analizar
ventajas/desventajas en cada caso).

Nota: todos los archivos se encuentran ordenados por código de productos. En cada detalle
puede venir 0 o N registros de un determinado producto.
*)
program ejer4_prac2;

uses
    SysUtils; 

const
    CANT_SUCURSALES = 30;
    valorAlto = 999;

type
    maestro = record
        codeProducto: integer;
        nombre: string;
        descripcion: string;
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
    regDetalles = array[1..CANT_SUCURSALES] of detalle;

procedure leer(var arch: fileDetalle; var dato: detalle);
begin
    if not eof(arch) then
        read(arch, dato)
    else
        dato.codeProducto := valorAlto;
end;

procedure minimo(var dets: detalles; var regs: regDetalles; var min: detalle);
var
    i: integer;
    posMin: integer;
begin
    min.codeProducto := valorAlto;
    posMin := 1;
    for i := 1 to CANT_SUCURSALES do begin
        if regs[i].codeProducto < min.codeProducto then begin
            min := regs[i];
            posMin := i;
        end;
    end;
    leer(dets[posMin], regs[posMin]);
end;

procedure editarArch(var VDetalles: detalles; var mae: fileMaestro);
var
    regM: maestro;
    VRegD: regDetalles;
    min: detalle;
    i: integer;
    filetxt:text;
begin
    reset(mae);

    seek(mae, 0);
    writeln('Archivo maestro original');
    while not eof(mae) do begin
        read(mae, regm);
        writeln('codeProducto: ', regm.codeProducto, ' stock disponible: ', regm.stockDiponible);
    end;

    for i := 1 to CANT_SUCURSALES do begin
        reset(VDetalles[i]);
        leer(VDetalles[i], VRegD[i]);
    end;
    
    seek(mae, 0);
    minimo(VDetalles, VRegD, min);
    while (min.codeProducto <> valorAlto) do begin
        read(mae, regm);
        while (regm.codeProducto <> min.codeProducto) do
            read(mae, regm);
        while (regm.codeProducto = min.codeProducto) do begin
            regm.stockDiponible := regm.stockDiponible - min.cantVendida;
            minimo(VDetalles, VRegD, min);
        end;
        seek(mae, filepos(mae) - 1);
        write(mae, regm);
    end;

    seek(mae, 0);
    assign(filetxt, 'productos.txt');
    rewrite(filetxt);
    writeln('Archivo maestro modificado');
    while not eof(mae) do begin
        read(mae, regm);
        if(regm.stockMinimo > regm.stockDiponible) then 
            writeln(filetxt, regm.precio:0:2, ' ',regm.stockDiponible, ' ', regm.nombre,' ', regm.descripcion);
        writeln('codeProducto: ', regm.codeProducto, ' stock disponible: ', regm.stockDiponible);
    end;
    close(mae);

    for i := 1 to CANT_SUCURSALES do
        close(VDetalles[i]);
end;

var
    mae: fileMaestro;
    VDetalles: detalles;
    i: integer;
begin
    assign(mae, 'maestro4');
    for i := 1 to CANT_SUCURSALES do
        assign(VDetalles[i], 'detalle' + IntToStr(i));
    
    editarArch(VDetalles, mae);
end.