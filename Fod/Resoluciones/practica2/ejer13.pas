program ejer13_prac2;

const
  VALOR_ALTO = 9999;

type
  maestro = record
    nro_usuario: integer;
    nombreUsuario: string;
    nombre: string;
    apellido: string;
    cantMailEnviados: integer;
  end;

  detalle = record
    nro_usuario: integer;
    cuentaDestino: string;
    cuerpoMensaje: string;
  end;

  fileMaestro = file of maestro;
  fileDetalle = file of detalle;

procedure leerDetalle(var arch: fileDetalle; var dato: detalle);
begin
  if not eof(arch) then
    read(arch, dato)
  else
    dato.nro_usuario := VALOR_ALTO;
end;

procedure actualizar(var mae: fileMaestro);
var
  det: fileDetalle;
  regm: maestro;
  regd: detalle;
  txt: text;
  totalMails, usuarioActual: integer;
  posMae: longint;
begin
  assign(det, 'detalleslog.dat');
  assign(txt, 'informe.txt');
  reset(mae);
  reset(det);
  rewrite(txt);

  leerDetalle(det, regd);
  while regd.nro_usuario <> VALOR_ALTO do begin

        usuarioActual := regd.nro_usuario;
        totalMails := 0;

        // contar mails del mismo usuario
        while regd.nro_usuario = usuarioActual do begin
            totalMails := totalMails + 1;
            leerDetalle(det, regd);
        end;

        // buscar usuario en el archivo maestro
        while not eof(mae) do begin
            read(mae, regm);
            if regm.nro_usuario = usuarioActual then begin

                regm.cantMailEnviados := regm.cantMailEnviados + totalMails;
                seek(mae, posfile(mae)-1);
                write(mae, regm);
                writeln(txt, regm.nro_usuario, ' .......... ', regm.cantMailEnviados);

            end;
        end;
        seek(mae, 0);
  end;

  close(mae);
  close(det);
  close(txt);   
end;

var
  mae: fileMaestro;
begin
  assign(mae, '/var/log/logmail.dat');
  actualizar(mae);
end.