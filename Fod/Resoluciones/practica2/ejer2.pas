program ejer2_prac2;

const 
    valorAlto = 9999;

type


    producto = record
        codeP: integer;
        nombreComercial: string;
        precioVenta: real;
        stockActual: integer;
        stockMin: integer;
    end;

    venta = record
        codeP: integer;
        cantVendida: integer;
    end;

    fileProductos = file of producto;
    fileVentas = file of venta;

procedure leer(var arch: fileVentas; var dato: venta);
begin
    if not eof(arch) then 
        read(arch, dato)
    else 
        dato.codeP := valorAlto;
end;

procedure incisoB;
var
    mae: fileProductos;
    txtFile: text;
    regm: producto;

begin

    reset(mae);

    assign(txtFile, 'stock_minimo.txt');
    rewrite(txtFile);

    while not eof(mae) do begin
        read(mae, regm);
        if regm.stockActual < regm.stockMin then
            writeln(txtFile, regm.codeP, '  ', regm.precioVenta:0:2, ' ', regm.stockActual, ' ', regm.stockMin, ' ', regm.nombreComercial);
    end;

    close(mae);
    close(txtFile);
end;

var

    regd: venta;
    regm: producto;
    mae: fileProductos;
    det: fileVentas;
    total, aux: integer;

begin

    assign(mae, 'maestroProductos');
    assign(det, 'detalleVentas');

    reset(mae);
    reset(det);

    leer(det, regd);
    
    if not eof(mae) then
        read(mae, regm);

    while (regd.codeP <> valorAlto) do begin
        aux := regd.codeP;
        total := 0;

        while (regm.codeP <> aux) do
            read(mae, regm);

        while (regd.codeP = aux) do begin
            total := total + regd.cantVendida;
            leer(det, regd);
        end;

        regm.stockActual := regm.stockActual - total;
        seek(mae, filepos(mae) - 1);

        write(mae, regm);

        if not eof(mae) then
            read(mae, regm);
    end;

    close(mae);
    close(det);

    incisoB;
end.