program ejer1_part2;

type  
        maestro = record
            codeProc : integer;
            nombre_comercial:string;
            precio_venta:real;
            stock_actual:integer;
            stock_minimo:integer;
        end;

        detalle = record

            codeProc:integer;
            cant_uni_vendidas:integer;

        end;

        fileMaestro = file of maestro;
        fileDetalle = file of detalle;


procedure actualizar(var mae: fileMaestro; var det: fileDetalle);
var
    regm: maestro;
    regd: detalle;
begin
    reset(mae);
    reset(det);
    
    while not eof(mae) do begin

        read(mae, regm);
        seek(det, 0); 

        while not eof(det) do begin

            read(det, regd);

            if regm.codeProc = regd.codeProc then
                regm.stock_actual := regm.stock_actual - regd.cant_uni_vendidas;

        end;

        seek(mae, filepos(mae) - 1); 

        write(mae, regm); 

    end;
    
    close(mae);
    close(det);
end;


//b

procedure actualizarB(var mae: fileMaestro; var det: fileDetalle);
var
    regm: maestro;
    regd: detalle;
    encontre : boolean;
begin

    reset(mae);
    reset(det);
    
    while not eof(mae) do begin
        encontre = false;
        read(mae, regm);
        seek(det, 0); 

        while not eof(det) and ( not encontre ) do begin

            read(det, regd);

            if regm.codeProc = regd.codeProc then begin
                regm.stock_actual := regm.stock_actual - regd.cant_uni_vendidas;
                encontre:= true;
            end

        end;

        seek(mae, filepos(mae) - 1); 

        write(mae, regm); 

    end;
    
    close(mae);
    close(det);
end;



var

    mae:fileMaestro;
    det:fileDetalle;

begin
    

    assign(mae, 'file_maestro');

    assign(det, 'file_detalle');


    actualizar(mae, det);



end;