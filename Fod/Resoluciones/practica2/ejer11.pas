program ejer11_prac2;
const
  VALOR_ALTO = 'ZZZZ';
  MAX_CAT = 15;

type
  maestro = record
    departamento: string[20];
    division: integer;
    numEmpleado: integer;
    categoria: integer;
    cantHoras: real;
  end;

  fileMaestro = file of maestro;
  vectorHoras = array[1..MAX_CAT] of real;

procedure leer(var arch: fileMaestro; var dato: maestro);
begin
  if not eof(arch) then
    read(arch, dato)
  else
    dato.departamento := VALOR_ALTO;
end;

procedure cargarVectorHoras(var v: vectorHoras);
var
  txtfile: text;
  cat: integer;
  valor: real;
begin
  assign(txtfile, 'texto.txt');
  reset(txtfile);
  while not eof(txtfile) do
  begin
    readln(txtfile, cat, valor);
    v[cat] := valor;
  end;
  close(txtfile);
end;

var
  mae: fileMaestro;
  regm: maestro;
  v: vectorHoras;
  deptoActual: string;
  divActual, empleadoActual: integer;
  totalHorasDep, totalHorasDiv, totalHorasEmpleado: real;
  montoDep, montoDiv, importe: real;
begin
  cargarVectorHoras(v);

  assign(mae, 'maestro.dat');
  reset(mae);
  leer(mae, regm);

  while regm.departamento <> VALOR_ALTO do
  begin
    deptoActual := regm.departamento;
    writeln('Departamento: ', deptoActual);
    totalHorasDep := 0;
    montoDep := 0;

    while (regm.departamento = deptoActual) do
    begin
      divActual := regm.division;
      writeln('  División: ', divActual);
      totalHorasDiv := 0;
      montoDiv := 0;

      while (regm.departamento = deptoActual) and (regm.division = divActual) do
      begin
        empleadoActual := regm.numEmpleado;
        totalHorasEmpleado := 0;

        while (regm.departamento = deptoActual) and (regm.division = divActual) and (regm.numEmpleado = empleadoActual) do
        begin
          totalHorasEmpleado := totalHorasEmpleado + regm.cantHoras;
          leer(mae, regm);
        end;

        importe := totalHorasEmpleado * v[regm.categoria];

        writeln('    Empleado: ', empleadoActual,
                ' | Total Horas: ', totalHorasEmpleado:0:2,
                ' | Importe: $', importe:0:2);

        totalHorasDiv := totalHorasDiv + totalHorasEmpleado;
        montoDiv := montoDiv + importe;
      end;

      writeln('  Total horas división: ', totalHorasDiv:0:2);
      writeln('  Monto total división: $', montoDiv:0:2);
      writeln;

      totalHorasDep := totalHorasDep + totalHorasDiv;
      montoDep := montoDep + montoDiv;
    end;

    writeln('Total horas departamento: ', totalHorasDep:0:2);
    writeln('Monto total departamento: $', montoDep:0:2);
    writeln('-------------------------------------------');
  end;

  close(mae);
end.
