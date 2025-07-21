program ejer19_prac2;

const
    CANT_DETALLES = 50;
    VALOR_ALTO = 9999999;

type
    nacimiento = record
        nroPartida: integer;
        nombre: string;
        apellido: string;
        calle: string;
        nro: integer;
        piso: string;
        depto: string;
        ciudad: string;
        matriculaMedico: integer;
        nombreMadre: string;
        apellidoMadre: string;
        dniMadre: integer;
        nombrePadre: string;
        apellidoPadre: string;
        dniPadre: integer;
    end;

    fallecimiento = record
        nroPartida: integer;
        dni: integer;
        nombre: string;
        apellido: string;
        matriculaMedico: integer;
        fecha: string;
        hora: string;
        lugar: string;
    end;

    persona = record
        nroPartida: integer;
        nombre: string;
        apellido: string;
        calle: string;
        nro: integer;
        piso: string;
        depto: string;
        ciudad: string;
        matriculaMedicoNac: integer;
        nombreMadre: string;
        apellidoMadre: string;
        dniMadre: integer;
        nombrePadre: string;
        apellidoPadre: string;
        dniPadre: integer;
        fallecio: boolean;
        matriculaMedicoFall: integer;
        fechaFalle: string;
        horaFalle: string;
        lugarFalle: string;
    end;

    fileNac = file of nacimiento;
    fileFall = file of fallecimiento;
    fileMaestro = file of persona;

    arrayNac = array[1..CANT_DETALLES] of fileNac;
    arrayFall = array[1..CANT_DETALLES] of fileFall;
    arrayRegNac = array[1..CANT_DETALLES] of nacimiento;
    arrayRegFall = array[1..CANT_DETALLES] of fallecimiento;

procedure leerNac(var f: fileNac; var r: nacimiento);
begin
    if not eof(f) then
        read(f, r)
    else
        r.nroPartida := VALOR_ALTO;
end;

procedure leerFall(var f: fileFall; var r: fallecimiento);
begin
    if not eof(f) then
        read(f, r)
    else
        r.nroPartida := VALOR_ALTO;
end;

procedure minimoNac(var v: arrayNac; var r: arrayRegNac; var min: nacimiento);
var i, pos: integer;
begin
    min.nroPartida := VALOR_ALTO;
    for i := 1 to CANT_DETALLES do
        if r[i].nroPartida < min.nroPartida then begin
            min := r[i];
            pos := i;
        end;
    if min.nroPartida <> VALOR_ALTO then
        leerNac(v[pos], r[pos]);
end;

procedure minimoFall(var v: arrayFall; var r: arrayRegFall; var min: fallecimiento);
var i, pos: integer;
begin
    min.nroPartida := VALOR_ALTO;
    for i := 1 to CANT_DETALLES do
        if r[i].nroPartida < min.nroPartida then begin
            min := r[i];
            pos := i;
        end;
    if min.nroPartida <> VALOR_ALTO then
        leerFall(v[pos], r[pos]);
end;

procedure crearMaestro(var mae: fileMaestro);
var
    vn: arrayNac;
    vf: arrayFall;
    rn: arrayRegNac;
    rf: arrayRegFall;
    minNac: nacimiento;
    minFall: fallecimiento;
    p: persona;
    txt: text;
    i: integer;
begin
    // Asignar, abrir y leer nacimientos y fallecidos.
    for i := 1 to CANT_DETALLES do begin
        assign(vn[i], 'nac' + IntToStr(i));
        reset(vn[i]);
        leerNac(vn[i], rn[i]);

        assign(vf[i], 'fall' + IntToStr(i));
        reset(vf[i]);
        leerFall(vf[i], rf[i]);
    end;

    assign(mae, 'maestro');
    rewrite(mae);
    assign(txt, 'personas.txt');
    rewrite(txt);

    minimoNac(vn, rn, minNac);
    minimoFall(vf, rf, minFall);

    while minNac.nroPartida <> VALOR_ALTO do begin
        // Armar registro maestro
        p.nroPartida := minNac.nroPartida;
        p.nombre := minNac.nombre;
        p.apellido := minNac.apellido;
        p.calle := minNac.calle;
        p.nro := minNac.nro;
        p.piso := minNac.piso;
        p.depto := minNac.depto;
        p.ciudad := minNac.ciudad;
        p.matriculaMedicoNac := minNac.matriculaMedico;
        p.nombreMadre := minNac.nombreMadre;
        p.apellidoMadre := minNac.apellidoMadre;
        p.dniMadre := minNac.dniMadre;
        p.nombrePadre := minNac.nombrePadre;
        p.apellidoPadre := minNac.apellidoPadre;
        p.dniPadre := minNac.dniPadre;
        //si esta muerto: 
        if minFall.nroPartida = minNac.nroPartida then begin
            p.fallecio := true;
            p.matriculaMedicoFall := minFall.matriculaMedico;
            p.fechaFalle := minFall.fecha;
            p.horaFalle := minFall.hora;
            p.lugarFalle := minFall.lugar;
            minimoFall(vf, rf, minFall);
        end
        else
            p.fallecio := false;

        write(mae, p);

        // Escribir en txt
        writeln(txt, 'Partida:', p.nroPartida, ', Nombre: ', p.nombre, ' ', p.apellido);

        minimoNac(vn, rn, minNac);
    end;

    close(mae);
    close(txt);

    for i := 1 to CANT_DETALLES do begin
        close(vn[i]);
        close(vf[i]);
    end;
end;

var
    mae: fileMaestro;
begin
    crearMaestro(mae);
end.