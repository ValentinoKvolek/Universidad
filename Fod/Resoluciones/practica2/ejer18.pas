program ejer18_prac2;

type

    maestro = record
        codeLocalidad:integer;
        nombreLocalidad:string;
        codeMunicipio:integer;
        nombreMunicipio:integer;
        codeHospital:integer;
        nombreHospital:integer;
        fecha:integer;
        cantCasosPos:integer;
    end;
    //ordenado por localidad, luego municipio y luego hospital.
    fileMaestro = file of maestro;

var
    mae:fileMaestro;
    regm:maestro;
    cantCasosL,cantCasosM,cantCasosH, cantTotal:integer;
    ActualL,ActualM,ActualH:string;
    txt:text;
begin
    cantTotal:=0;

    assign(mae,'maestro');
    reset(mae);
    assign(txt, 'file text');
    rewrite(txt);

    read(mae,regm);

    while not eof(mae) do begin
        
        ActualL:=regm.nombreLocalidad;
        cantCasosL:=0;

        writeln('localidad : ', ActualL);
        
        while(ActualL = regm.nombreLocalidad) do begin

            ActualM:=regm.nombreMunicipio;
            cantCasosM:=0;

            writeln('municipio:', ActualM);
            
            while(ActualL = regm.nombreLocalidad) and(ActualM = regm.nombreMunicipio) do begin
                
                ActualH:= regm.nombreHospital;
                cantCasosH:=0;
                
                while(ActualL = regm.nombreLocalidad) and(ActualM = regm.nombreMunicipio) and (ActualH = regm.nombreHospital) do begin
                    cantCasosH:=cantCasosH + regm.cantCasosPos;
                    read(mae,regm);
                end;

                writeln('nombre hospital:' ActualH, ' cantidad de casos : '. cantCasosH);
                cantCasosM:= cantCasosM + cantCasosH;
                

            end;
            
            writeln('cantidad de caso del municipio: ', cantCasosM);

            if cantCasosM > 1500 then
                writeln(txt, ActualL, ' ', ActualM, ' ', cantCasosM);


            cantCasosL:=cantCasosL+cantCasosM;
            
        end;

        writeln('cantidad de casos de la localidad: ', cantCasosL);

        cantTotal:=cantCasosL;
    end;

    writeln('cantidad de casos totales en la provincia: ', cantTotal); 

    close(mae);

end;