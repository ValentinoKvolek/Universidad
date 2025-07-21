program ejer4_prac2;

type

    reg_flor = record
        nombre:string[45];
        code:integer;
    end;

    tArchFlores = file of reg_flor;

procedure agregarFlor(var fl :tArchFlores; nombre :string; code:integer);
var
    reg, regCabecera: reg_flor;
    pos:integer;
begin

    reset(fl);

    read(fl, regCabecera);  // Leo la cabecera
    if(regCabecera.code = 0) then begin
        seek(fl, FileSize(fl));
        reg.code:=code;
        reg.nombre:=nombre;
        write(fl, reg);
    end
    else begin

        pos:= regCabecera.code *-1;
        seek(fl, pos);
        read(fl,reg); // Leo el registro a reutilizar (para saber quién sigue en la pila)

        // actualizo el siguiente en el registro cabecera.
        seek(fl,0); 
        regCabecera.code := reg.code;
        write(fl, regCabecera);

        //escribo la nueva flor.
        seek(fl,pos);
        reg.code:=code;
        reg.nombre:=nombre;
        write(fl, reg); 

    end;

    writeln('se agrego correctamente');
    close(fl);
end;

procedure listar(var fl:tArchFlores);
var
    reg:reg_flor;
begin
    reset(fl);
    seek(fl,1);
    while(not eof(fl))do begin
        read(fl,reg);
        if(reg.code > 0) then
            writeln('codigo de flor: ', reg.code, ' nombre:', reg.nombre);
    end;
    close(fl);
end;

procedure eliminar(var fl:tArchFlores; flor:reg_flor);
var
    reg, aux: reg_flor;
    existe: boolean;
    pos:integer;
begin

    existe:=false;
    reset(fl);  


    //primero me fijo si esa flor a eliminar existe;
    while(not eof(fl))and (not existe )do begin

        read(fl,reg);   

        if(reg.code = flor.code) then begin
            existe:=true;
            pos:=FilePos(fl) - 1;
        end;

    end;

    //pregunto por que rezon salio
    if existe then begin
        
        seek(fl , 0);
        read(fl, aux); //me guardo el anterior libre para anidarlo.

        //En la posición eliminada, escribir el valor anterior de la cabecera
        reg.code:= aux.code;
        reg.nombre:='';
        seek(fl, pos); 
        write(fl, reg); 

        //me guardo el nuevo libre en la cabecera.
        seek(fl, 0);
        aux.code := -pos;
        write(fl,aux);

        writeln('flor borrada con exito.')
    end
    else
        writeln('la flor no existe');

    close(fl);
end;

var
    fl:tArchFlores;
    florDelete: reg_flor;
    nombre: string;
    code, input:integer;
    
begin  


    assign(fl, 'floresFile');

    writeln('|| (1) ELIMINAR || || (2) AGREGAR || || (3) LISTAR || || (4) CERRAR ||');
    readln(input);

    while(input<>4) do begin
    
        case input of 

            1: 
                begin
                    //5
                    writeln('ingrese el code de la flor a eliminar');
                    readln(florDelete.code);
                    writeln('ingrese el nombre de la flor a eliminar');
                    readln(florDelete.nombre);
                    eliminar(fl, florDelete);
                end;
            2: 
                begin
                    //4
                    writeln('ingrese nombre de la flor nueva');
                    readln(nombre);
                    writeln('ingrese el codigo de la flor nueva');
                    readln(code); 
                    agregarFlor(fl, nombre, code);
                end;
            3:
                begin
                //4b
                listar(fl);
                end;

        end;
        writeln('|| (1) ELIMINAR || || (2) AGREGAR || || (3) LISTAR || || (4) CERRAR ||');
        readln(input);

    end;
end.