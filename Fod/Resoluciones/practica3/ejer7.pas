(*


Se cuenta con un archivo que almacena información sobre especies de aves en vía
de extinción, para ello se almacena: código, nombre de la especie, familia de ave,
descripción y zona geográfica. El archivo no está ordenado por ningún criterio. Realice
un programa que permita borrar especies de aves extintas. Este programa debe
disponer de dos procedimientos:

a.​ Un procedimiento que dada una especie de ave (su código) marque la misma
como borrada (en caso de querer borrar múltiples especies de aves, se podría
invocar este procedimiento repetidamente)

b.​ Un procedimiento que compacte el archivo, quitando definitivamente las
especies de aves marcadas como borradas. Para quitar los registros se deberá
copiar el último registro del archivo en la posición del registro a borrar y luego
eliminar del archivo el último registro de forma tal de evitar registros duplicados.
i.​ Implemente una variante de este procedimiento de compactación del
archivo (baja física) donde el archivo se trunque una sola vez.

*)


program ejer7_prac7;

type

    aves = record
        code:integer;
        nombre:string;
        familia:string;
        descripcion:string;
        zona:string;
    end;
    
    fileMaestro = file of aves;

var 

    mae:fileMaestro;
    input:integer;


procedure eliminarEspecie(var mae:fileMaestro; input:integer);

var 
    existe : boolean;
    reg : aves;

begin
    existe:=false;

    reset(mae);

    while(not eof (mae))  and (not existe ) do begin

        read(mae, reg);

        if(reg.code = input) then begin

            reg.code:= -99;
            seek(mae, FilePos(mae)-1);
            write(mae, reg);
            existe:= true;

        end;    
    

    end;

    if(existe) then 
        writeln('especie eliminada con exito')
    else
        writeln('especie no encontrada');

    close(mae);
    
end;


procedure compactar(var mae: fileMaestro);

var

    pos:integer;
    reg, aux: aves;
begin

    reset(mae);
    
    while(not eof (mae)) do begin 

        read(mae, reg);

        if(reg.code < 0) then begin

            pos:=FilePos(mae)-1; //pos donde hay uno borrado

            seek(mae, filesize(mae)-1); //busco el ultimo;

            read(mae, aux); //me copio el ultimo.

            if(filepos(mae)-1 <> 0) then // veo si el ultimo no es el final del archivo.

                while(aux.code < 0) do begin //Puede pasar que los ultimos tambien se borren, cuando esto no pase sale del while.

                        seek(mae, filesize(mae)-1);  //me vuelvo a pos en el ultimo 
                        truncate(mae); //trunco.

                        seek(mae, filesize(mae)-1);//me pos en le nuevo ultimo 
                        read(mae, aux);

                end;
        

            seek(mae, pos); //voy a la posicion que elimine logicamente.

            write(mae, aux); //copio ultimo en el borrado.

            seek(mae, FileSize(mae) - 1);   //importante moverte al ultimo lugar para truncar y evitar duplicado o que se trunque todo hasta la pos.
            truncate(mae);

            seek(mae, pos); //reposiciono para seguir recorriendo.

        end;
        
    end;

    close(mae);
    
end;

procedure compactari(var mae: fileMaestro);

var
    fileAux:fileMaestro;
    reg: aves;
begin

    assign(fileAux, 'auxFile');
    rewrite(fileAux);
    reset(mae);
    
    while(not eof (mae)) do begin   

        read(mae,reg);
        if(reg.code > 0) then 
            write(fileAux, reg);
        
    end;

    close(mae);
    close(fileAux);

    rename(fileAux, 'archivo_aves');
    rename(mae, 'archivo_aves_old');
    
end;

procedure imprimir(var mae: fileMaestro);
var
    reg: aves;

begin

    reset(mae);

    while not eof(mae) do begin
        read(mae, reg);
        writeln('CODE: ', reg.code , ' NOMBRE:',  reg.nombre);
    end;

    close(mae);
    
end;

begin
    
    assign(mae, 'archivo_aves');
    
    writeln('| Ingrese la opcion que desee: |');

    writeln('|| (1):= ELIMINAR ESPECIE || (2):= COMPACTAR ARCH || (3): = IMPRIMIR ARCH COMPRIMIDO. || (4):= COMPACTAR (I) ARCH || (5):= CERRAR');
    readln(input);

    while(input <> 5) do begin
        case input of 
        
            1: 
                begin

                    writeln('ingrese el codigo de la especie que quiere borrar: ');
                    readln(input);
                    eliminarEspecie(mae, input);
                end;

            2:
                begin
                    compactar(mae);
                end;

            3:
                begin
                    imprimir(mae);
                end;

            4: 
                begin

                    compactari(mae);
                end;
                
        end;
            
        writeln('|| (1):= ELIMINAR ESPECIE || (2):= COMPACTAR ARCH || (3): = IMPRIMIR ARCH COMPRIMIDO. || (4):= COMPACTAR (I) ARCH || (5):= CERRAR');
        readln(input);

    end;

end.

