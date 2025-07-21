program test3;

type
    maestro = record
        nombreProv: string;
        cantPersona: integer;
        totalEncuestas: integer;
    end;

    detalle = record
        nombreProv: string;
        codeLocalidad: integer;
        cantAlfabetizados: integer;
        cantEncuestados: integer;
    end;

    fileMaestro = file of maestro;
    fileDetalle = file of detalle;

var
    regMae: maestro;
    regDet: detalle;
    mae: fileMaestro;
    det, det2: fileDetalle;

begin
    assign(mae, 'archivoMaestro3');
    assign(det, 'archivoDetalle1_3');
    assign(det2, 'archivoDetalle2_3');

    rewrite(mae);
    rewrite(det);
    rewrite(det2);

    // Cargar archivo maestro
    writeln('--- CARGA DE ARCHIVO MAESTRO ---');
    writeln('Ingrese nombre de provincia (FIN para terminar): ');
    readln(regMae.nombreProv);
    while (regMae.nombreProv <> 'FIN') do
    begin
        writeln('Ingrese cantidad de personas: ');
        readln(regMae.cantPersona);
        writeln('Ingrese total de encuestas: ');
        readln(regMae.totalEncuestas);
        write(mae, regMae);
        writeln('Ingrese nombre de provincia (FIN para terminar): ');
        readln(regMae.nombreProv);
    end;

    // Cargar archivo detalle 1
    writeln('--- CARGA DE ARCHIVO DETALLE 1 ---');
    writeln('Ingrese nombre de provincia (FIN para terminar): ');
    readln(regDet.nombreProv);
    while (regDet.nombreProv <> 'FIN') do
    begin
        writeln('Ingrese código de localidad: ');
        readln(regDet.codeLocalidad);
        writeln('Ingrese cantidad de alfabetizados: ');
        readln(regDet.cantAlfabetizados);
        writeln('Ingrese cantidad de encuestados: ');
        readln(regDet.cantEncuestados);
        write(det, regDet);
        writeln('Ingrese nombre de provincia (FIN para terminar): ');
        readln(regDet.nombreProv);
    end;

    // Cargar archivo detalle 2
    writeln('--- CARGA DE ARCHIVO DETALLE 2 ---');
    writeln('Ingrese nombre de provincia (FIN para terminar): ');
    readln(regDet.nombreProv);
    while (regDet.nombreProv <> 'FIN') do
    begin
        writeln('Ingrese código de localidad: ');
        readln(regDet.codeLocalidad);
        writeln('Ingrese cantidad de alfabetizados: ');
        readln(regDet.cantAlfabetizados);
        writeln('Ingrese cantidad de encuestados: ');
        readln(regDet.cantEncuestados);
        write(det2, regDet);
        writeln('Ingrese nombre de provincia (FIN para terminar): ');
        readln(regDet.nombreProv);
    end;

    close(mae);
    close(det);
    close(det2);
end.
