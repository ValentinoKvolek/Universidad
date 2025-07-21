program ejer6_prac3;
const
    VALOR_ALTO=999;
type 
    prenda = record
        cod_prenda:integer;
        descripcion: string;
        colores:string;
        tipo_prenda:string;
        stock:integer;
        precio_unitario:real;
    end;

    delete = record
        cod_prenda:integer;
    end;

    fileDelete =file of delete;
    filePrenda =file of prenda;


procedure bajaPrendas(var fp:filePrenda; var fd:fileDelete);
var
    regP:prenda;
    regD:delete;
    pos:integer;
    existe : boolean;
begin
    existe:=false;
    pos:=0;
    reset(fp);
    reset(fd);

    read(fd,regD);

    while(not eof(fd)) do begin //mientras haya codes de ropa para eliminar 
        
        existe:=false;
        pos:=0;
        seek(fp, 0); //vuelvo al inicio por que no tiene orden.

        while(not existe ) and (not eof(fp)) do begin

            read(fp, regP);

            if(regP.cod_prenda = regD.cod_prenda) then begin
                existe:=true;
                pos:=(FilePos(fp)-1);
            end;

        end;

        if (existe) then begin

            seek(fp, pos);
            read(fp, regP);
            regP.stock:= -regP.stock;
            seek(fp, pos);
            write(fp,regP);

            writeln('eliminado con exito')

        end
        else writeln('prende no encontrada');

        read(fd, regD);

    end;

    close(fp); close(fd);
end;

procedure efectivizar(var fp:filePrenda);
var
    aux:filePrenda;
    regP:prenda;
begin
    
    reset(fp);

    assign(aux, 'archivoAux');
    rewrite(aux);

    while(not eof(fp)) do begin
        read(fp, regP);
        if(regP.stock >= 0) then 
            write(aux, regP);
    end;

    writeln('archivo prendas viejo:');
    seek(fp,0);
    while(not eof(fp)) do begin
        read(fp, regP);
        writeln('code: ', regP.cod_prenda ,'||', regP.stock);
    end;

    writeln('archivo prendas nuevo:');
    seek(aux,0);
    while(not eof(aux)) do begin
        read(aux, regP);
        writeln(regP.cod_prenda,'||', regP.stock);
    end;

    close(fp);
    close(aux); //aca se cierran antes de cambiar la ruta de lo arch


    rename(fp,'archivoPrendas_old');
    rename(aux, 'archivoPrendas');

end;

var
    fp:filePrenda;
    fd:fileDelete;

begin

    assign(fp,'archivoPrendas');
    assign(fd,'archivoPrendasEliminadas');

    bajaPrendas(fp, fd);

    efectivizar(fp);

end.
