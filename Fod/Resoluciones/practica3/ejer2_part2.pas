program ejer2_part2;

type 
     
        maestro = record
            codeLocalidad: integer;
            numMesa:integer;
            cantVotos:integer;
        end;
        
        localidad = record

            codeLocalidad: integer;
            cantTotal : integer;
        end;

        fileMaestro = file of maestro;

        fileLocalidad = file of localidad; //file que voy a usar para guardarme al localidad que ya procece.


var

    mae: fileMaestro;
    loc: fileLocalidad;

    regm:maestro;
    regl:localidad;

    existe:boolean;

begin

    assign(mae, 'file_maestro');
    reset(mae);

    assign(loc, 'file_loc');
    rewrite(loc);


    while(not eof(mae)) do begin

        read(mae, regm);
        existe:=false;
        seek(loc, 0);

        while(not eof(loc)) and (not existe) do begin
            read(loc, regl);
            if(regl.codeLocalidad = regm.codeLocalidad) then
                existe := true;
        end;
        if(existe) then begin
            regl.cantTotal:=regl.cantTotal +  regm.cantVotos;
            seek(loc, filepos(loc)-1);
            write(loc, regl);
        end
        else begin
            regl.codeLocalidad:=regm.codeLocalidad;
            regl.cantTotal:=regm.cantVotos;
            seek(loc, filesize(loc));
            write(loc, regl);
        end;

    end;

    seek(loc,  0);

    while not eof(loc) do begin
        read(loc, regl);
        writeln(' code localidad: ', regl.codeLocalidad , ' cantidad de votos : ', regl.cantTotal);
    end;


    close(mae);
    close(loc);


end.