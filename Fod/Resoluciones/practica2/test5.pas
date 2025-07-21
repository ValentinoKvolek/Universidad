program crearDetalles;

uses
    SysUtils;

const
    CANT_MAQUINAS = 5;

type
    detalle = record
        cod_usuario: integer;
        fecha: string[10];  // formato: 'YYYY-MM-DD'
        tiempo_sesion: real;
    end;

    fileDetalle = file of detalle;

procedure escribirDetalle(nombreArchivo: string; registros: array of detalle);
var
    arch: fileDetalle;
    i: integer;
begin
    assign(arch, nombreArchivo);
    rewrite(arch);
    for i := 0 to High(registros) do
        write(arch, registros[i]);
    close(arch);
end;

var
    r1, r2, r3, r4, r5: array of detalle;

begin
    // Reservar memoria para los arrays
    SetLength(r1, 3);
    SetLength(r2, 3);
    SetLength(r3, 3);
    SetLength(r4, 2);
    SetLength(r5, 2);

    // detalle1
    r1[0].cod_usuario := 101; r1[0].fecha := '2025-04-01'; r1[0].tiempo_sesion := 1.5;
    r1[1].cod_usuario := 102; r1[1].fecha := '2025-04-01'; r1[1].tiempo_sesion := 2.0;
    r1[2].cod_usuario := 103; r1[2].fecha := '2025-04-02'; r1[2].tiempo_sesion := 1.0;
    escribirDetalle('detalle1', r1);

    // detalle2
    r2[0].cod_usuario := 101; r2[0].fecha := '2025-04-01'; r2[0].tiempo_sesion := 2.0;
    r2[1].cod_usuario := 101; r2[1].fecha := '2025-04-02'; r2[1].tiempo_sesion := 1.5;
    r2[2].cod_usuario := 104; r2[2].fecha := '2025-04-03'; r2[2].tiempo_sesion := 3.0;
    escribirDetalle('detalle2', r2);

    // detalle3
    r3[0].cod_usuario := 102; r3[0].fecha := '2025-04-01'; r3[0].tiempo_sesion := 1.5;
    r3[1].cod_usuario := 103; r3[1].fecha := '2025-04-02'; r3[1].tiempo_sesion := 2.5;
    r3[2].cod_usuario := 105; r3[2].fecha := '2025-04-04'; r3[2].tiempo_sesion := 1.0;
    escribirDetalle('detalle3', r3);

    // detalle4
    r4[0].cod_usuario := 101; r4[0].fecha := '2025-04-02'; r4[0].tiempo_sesion := 2.0;
    r4[1].cod_usuario := 102; r4[1].fecha := '2025-04-03'; r4[1].tiempo_sesion := 1.0;
    escribirDetalle('detalle4', r4);

    // detalle5
    r5[0].cod_usuario := 101; r5[0].fecha := '2025-04-01'; r5[0].tiempo_sesion := 0.5;
    r5[1].cod_usuario := 104; r5[1].fecha := '2025-04-03'; r5[1].tiempo_sesion := 1.5;
    escribirDetalle('detalle5', r5);

    writeln('Archivos detalle creados correctamente.');
end.
