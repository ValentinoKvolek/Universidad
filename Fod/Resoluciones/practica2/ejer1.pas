(* 

Una empresa posee un archivo con información de los ingresos percibidos por diferentes
empleados en concepto de comisión, de cada uno de ellos se conoce: código de empleado,
nombre y monto de la comisión. 

La información del archivo se encuentra ordenada por
código de empleado y cada empleado puede aparecer más de una vez en el archivo de
comisiones.

Realice un procedimiento que reciba el archivo anteriormente descrito y lo compacte.
En consecuencia, deberá generar un nuevo archivo en el cual, cada empleado aparezca una
única vez con el valor total de sus comisiones.

NOTA: No se conoce a priori la cantidad de empleados. Además, el archivo debe ser
recorrido una única vez. 

*)


program ejer1_prac2;

type

    empleado  = record
        codeEmp:integer; //ordenado por code de empleado
        nombre:string;
        montoComision:real;
    end;

    empleadoCompacto = record
        codeEmp:integer;
        totalComision: real;
    end;

    fileEmp =file of empleado;

    fileEmpCompacto =file of empleadoCompacto;


procedure incisoA (var mae:fileEmpCompacto; var det:fileEmp);
var
    regm:empleadoCompacto;
    regd:empleado;
    codeAct:integer;

begin

    //abro el detalle.
    reset(det);

    reset(mae);

    read(det,regd);

    while not eof(det) do begin

        codeAct:=regd.codeEmp;

        while(regd.codeEmp = codeAct) and not eof(det) do begin

            //mientras sea el mismo empleado actualizo la comision.
            regm.totalComision:= regm.totalComision + regd.montoComision;

            read(det, regd);

        end;

        //si son iguales
        if(regd.codeEmp = codeAct) then 
            regm.totalComision:= regm.totalComision + regd.montoComision;
            
        //Escribo antes de fijarme si salio por que es el ultimo archivo y ademas no son iguales para no perder informacion.
        regm.codeEmp:=codeAct;
        write(mae, regm);
        regm.totalComision:=0;

        if(eof(det)) then begin
            regm.totalComision:= regd.montoComision;
            regm.codeEmp:= regd.codeEmp;
            write(mae,regm);
        end;
        
    end;

    writeln('archivo detalle: ');
    seek(det, 0);

    while not eof(det) do begin
        read(det, regd);

        writeln('codigo: ', regd.codeEmp, ' ', 'comision: ', regd.montoComision :0:2);

    end;
    
    writeln('archivo nuevo: ');
    seek(mae, 0);
    while not eof(mae) do begin
        read(mae, regm);

        writeln('codigo: ', regm.codeEmp, ' ', 'total: ', regm.totalComision :0:2);

    end;

    close(mae);
    close(det);

end;

var
    mae: fileEmpCompacto;
    det: fileEmp;
    regm:empleadoCompacto;
    regd:empleado;
begin
    
    assign(mae,'EmpC');
    assign(det, 'Emp');

    //creo el maestro
    rewrite(mae);

    incisoA(mae,det);

end.
