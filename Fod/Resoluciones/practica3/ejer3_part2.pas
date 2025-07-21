(*
Suponga que trabaja en una oficina donde está montada una LAN (red local). La
misma fue construida sobre una topología de red que conecta 5 máquinas entre sí y
todas las máquinas se conectan con un servidor central. Semanalmente cada
máquina genera un archivo de logs informando las sesiones abiertas por cada usuario
en cada terminal y por cuánto tiempo estuvo abierta. Cada archivo detalle contiene
los siguientes campos: cod_usuario, fecha, tiempo_sesion. 
Debe realizar un procedimiento que reciba los archivos detalle y genere un archivo maestro con los
siguientes datos: cod_usuario, fecha, tiempo_total_de_sesiones_abiertas.
Notas:
●​ Los archivos detalle no están ordenados por ningún criterio.
●​ Un usuario puede iniciar más de una sesión el mismo día en la misma máquina,
o inclusive, en diferentes máquinas.

*)

program ejer3_prac2;

const 
    VALOR_ALTO =  999;

type

    detalle = record
        cod_usuario: integer;
        fecha:string;
        tiempo_sesion:real;
    end;

    maestro = record
        cod_usuario:integer;
        fecha:string;
        tiempo_total_de_sesiones_abiertas:real;
    end;

    fileDetalle = file of detalle;

    fileMaestro = file of maestro;


var

    mae:fileMaestro;

    vd: vectorDetalles;

    regm:maestro;

    regd:detalle;

    i:integer;

    min:detalle;

begin

    assing(mae, 'maestro');
    rewrite(mae);

    for i:= 1 to 5 do begin  //por cada detalle.
        assing(vd[i], 'detalle', i);
        reset(vd[i]);

        while not eof(vd[i]) do begin
            
            read(v[i], regd);

            existe:= false;

            seek(mae, 0);

            while not eof(mae) and (not existe) do begin
                read(mae, regm);
                if(regm.cod_usuario = vr[i].cod_usuario) then
                    existe:= true;
            end;

            if(existe) then begin
                regm.tiempo_total_de_sesiones_abiertas:= regm.tiempo_total_de_sesiones_abiertas + regd.tiempo_sesion;
                seek(mae, filepos(mae)-1);
                write(mae, regm);
            end
            else begin
                write(mae, regd);
            end;
        end;

        close(v[i]); // cuando termine de recorrer cierro

    end;

    close(mae);

end;