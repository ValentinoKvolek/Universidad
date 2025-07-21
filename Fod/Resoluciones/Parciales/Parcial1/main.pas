program parcial1;

const cantArchivos = 30; VALOR_ALTO:999;

type

    venta = record;
        cod_farmaco:integer;
        nombre:String;
        fecha:String;
        cant_vendida:integer;
        forma_pago:integer;
    end;
    //Los archivos de venta estan ordenados por codigo de farmaco y por la fecha en ese orden.


    fileVentas = file of venta;
    vectorFiles = array [1.. cantArchivos] of fileVentas;
    vectorRegs = array [1.. cantArchivos] of venta;



procedure leer(var arch:fileVentas; var dato:venta);
begin
    if not eof(arch) then
        read(arch, dato);
    else
    begin
        dato.cod_farmaco := VALOR_ALTO;
    end;
end;

procedure minimo(var vf:vectorFiles; var vr:vectorRegs, var min:venta);
var
    pos,i:integer;
begin
    min.code:= VALOR_ALTO;
    for i := 1 to cantArchivos do begin
        if(vr[i].cod_farmaco < min.code) or (vr[i].cod_farmaco = min.code) and (vr[i].fecha < min.code.fecha)  then begin
            min := vr[i];
            pos :=i;
        end;
    end;
    if(min.cod_farmaco <> VALOR_ALTO) then
        leer(vf[pos], vr[pos]);
end;

procedure informar(var vf:vectorFiles; var vr: vectorRegs);
var
    i:integer;
    min:venta;
    cantTotal:integer;
    codActual:integer;
    fechaActual:String;
    cantMaximo, max:integer;
begin

    for i:=1 to cantArchivos do begin
        reset(vf[i]);
        leer(vf[i], vr[i]);
    end;

    minimo(vf, vr, min);
    
    while (min.cod_farmaco <> VALOR_ALTO) do begin

        cantTotal:=0;
        codActual:=min.cod_farmaco;
        fechaActual:=min.fecha;

        while(min.cod_farmaco = codActual) and (min.fecha = fechaActual) do begin
            cantTotal:= cantTotal+  min.cant_vendida;
            minimo(vf,vr,min);
        end;

        if(cantTotal > cantMaximo) then begin
            cantMaximo :=  cantTotal;
            max:codActual;
        end;
        
    end;

    writeln('el farmaco con mayor cantidad vendida fue', max);

    for i:=1 to cantArchivos do 
        close(vf[i]);
    
end;

procedure guardar_en_txt(var vf:vectorFiles; var vr:vectorRegs);
var
    cantTotal:integer;
    codActual:integer;
    fechaActual, nombre:String;
    txt = TEXT;
    min:ventas;
    
begin
    assign(txt, 'txt');
    rewrite(txt);

    for i:=1 to cantArchivos do begin
        reset(vf[i]);
        leer(vf[i], vr[i]);
    end;

    minimo(vf, vr, min);

    while (min.cod_farmaco <> VALOR_ALTO) do begin

        cantTotal:=0;
        codeActual:=min.cod_farmaco;
        fechaActual:=min.fecha;
        nombre:min.nombre;

        while(min.cod_farmaco = codActual) and (min.fecha = fechaActual) do begin
            cantTotal:= cantTotal +  min.cant_vendida;
            minimo(vf,vr,min);
        end;

        writeln(txt, codActual,' ', nombre,' ', fechaActual,' ', cantTotal);

    end;

    for i:=1 to cantArchivos do 
        close(vf[i]);

end;



