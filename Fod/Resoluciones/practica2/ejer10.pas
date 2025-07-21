program ejer10_ejer2;

const VALOR_ALTO =999;
type

    maestro = record
        codeProv:integer;
        codeLocalidad:integer;
        numMesa:integer;
        cantVoto:integer;
    end;

    fileMaestro = file of maestro;

procedure leer(var arch: fileMaestro; var dato: maestro);
begin
    if (not eof(arch)) then
        read(arch, dato)
    else
        dato.codeProv := VALOR_ALTO;
end;

var
    mae:fileMaestro;
    regm:maestro;
    actualProv,actualLocalidad:integer;
    totalProv,totalLocalidad,totalVotos:integer;
begin
    assign(mae, 'maestro');
    reset(mae);
    leer(mae, regm);
    totalVotos:=0;
    while regm.codeProv <> VALOR_ALTO do begin
        actualProv:=regm.codeProv;
        totalProv:=0;
        writeln('code provincia', actualProv);
        while(actualProv = regm.codeProv) do begin
            actualLocalidad:= regm.codeLocalidad;
            totalLocalidad:=0;
            while(actualLocalidad = regm.codeLocalidad) do begin
                totalLocalidad:= totalLocalidad +  regm.cantVoto;
                leer(mae,regm);
            end;
            writeln('code localidad:', totalLocalidad);
            totalProv:=totalProv + totalLocalidad;
        end;
        totalVotos:=totalVotos+totalProv;
        writeln('total provincia: ', totalProv);
    end;
    writeln('total general votos:', totalVotos);
    close(mae);
end;
      