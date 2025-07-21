program CreateTestMaestro;

type
    maestro = record
        codeLocalidad: integer;
        numMesa: integer;
        cantVotos: integer;
    end;

    fileMaestro = file of maestro;

var
    mae: fileMaestro;
    regm: maestro;
    i: integer;

begin
    { Asignar y crear el archivo maestro }
    assign(mae, 'file_maestro');
    rewrite(mae);

    { Generar 10 registros de prueba }
    for i := 1 to 10 do
    begin
        { Simular datos no ordenados }
        case i of
            1: begin
                regm.codeLocalidad := 1;
                regm.numMesa := 101;
                regm.cantVotos := 150;
            end;
            2: begin
                regm.codeLocalidad := 2;
                regm.numMesa := 201;
                regm.cantVotos := 200;
            end;
            3: begin
                regm.codeLocalidad := 1;
                regm.numMesa := 102;
                regm.cantVotos := 175;
            end;
            4: begin
                regm.codeLocalidad := 3;
                regm.numMesa := 301;
                regm.cantVotos := 300;
            end;
            5: begin
                regm.codeLocalidad := 2;
                regm.numMesa := 202;
                regm.cantVotos := 180;
            end;
            6: begin
                regm.codeLocalidad := 4;
                regm.numMesa := 401;
                regm.cantVotos := 250;
            end;
            7: begin
                regm.codeLocalidad := 1;
                regm.numMesa := 103;
                regm.cantVotos := 160;
            end;
            8: begin
                regm.codeLocalidad := 3;
                regm.numMesa := 302;
                regm.cantVotos := 270;
            end;
            9: begin
                regm.codeLocalidad := 5;
                regm.numMesa := 501;
                regm.cantVotos := 220;
            end;
            10: begin
                regm.codeLocalidad := 2;
                regm.numMesa := 203;
                regm.cantVotos := 190;
            end;
        end;

        { Escribir el registro en el archivo }
        write(mae, regm);
    end;

    { Cerrar el archivo }
    close(mae);
    writeln('Archivo file_maestro creado con exito.');
end.