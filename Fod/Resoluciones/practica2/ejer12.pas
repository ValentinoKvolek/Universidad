program ejer12_prac2;
const VALOR_ALTO = 9999;
type

    maestro = record    
        anio:integer;
        mes:integer;
        dia:integer;
        idUsuario:integer;
        tiempoAcceso:integer;
    end;
    //ordenado: anio ,mes, dia e usuario;
    fileMaestro = file of maestro;

procedure leer(var arch:fileMaestro; var dato:maestro);
begin
    if(not eof(arch)) then
        read(arch, dato)
    else
        dato.anio:=VALOR_ALTO;
end;

procedure imprimir(var mae:fileMaestro; input:integer);
var
  regm: maestro;  
  totalTiempoAccesoAnio, totalTiempoAccesoMes, totalTiempoAccesoDia, totalTiempoAccesoId:integer;
  anioActual, mesActual,diaActual,idActual:integer;
  encontrado:boolean;
begin  

    reset(mae);
    leer(mae, regm);
    encontrado:=false;

    while(regm.anio <> VALOR_ALTO) do begin
        
        if(regm.anio = input) then
        begin
            // una vez encontre el anio: 
            anioActual := regm.anio;
            totalTiempoAccesoAnio:=0;
            writeln('anio:', anioActual);
            while(regm.anio = anioActual) do begin
                mesActual:= regm.mes;
                totalTiempoAccesoMes:=0;
                writeln('  -mes: ', mesActual);
                while(regm.mes = mesActual) and (regm.anio = anioActual) do begin
                    diaActual:=regm.dia;
                    idActual:=regm.idUsuario;
                    totalTiempoAccesoDia:=0;
                    writeln('dia:', diaActual);

                    while(diaActual= regm.dia)  and (regm.mes = mesActual) and (regm.anio = anioActual) do begin
                        idActual:=regm.idUsuario;
                        totalTiempoAccesoId:=0;
                        while(idActual = regm.idUsuario) and(diaActual= regm.dia)  and (regm.mes = mesActual) and (regm.anio = anioActual) do begin
                            totalTiempoAccesoId:=totalTiempoAccesoId+ regm.tiempoAcceso;
                            leer(mae,regm);
                        end;

                        writeln('idUsuario ', idActual,' Tiempo Total de acceso en el dia' diaActual, 'mes', mesActual, ' fue de:', totalTiempoAccesoId);
                        totalTiempoAccesoDia:=totalTiempoAccesoDia+totalTiempoAccesoId;
                    end;

                    writeln('total tiempo de acceso del dia', diaActual, ' mes:', mesActual, 'fue de:', totalTiempoAccesoDia);
                    totalTiempoAccesoMes:=totalTiempoAccesoMes+totalTiempoAccesoDia;
                end;
                
                writeln('tiempo de acceso total del mes', mesActual ,'fue de:', totalTiempoAccesoMes);
                totalTiempoAccesoAnio:=totalTiempoAccesoAnio+totalTiempoAccesoMes;
            end;
            writeln('tiempo de acceso totel en todo el anio: ', totalTiempoAccesoAnio);
        end
        else
            leer(mae,regm);
    end;

    if not encontrado then
        writeln('Año no encontrado en el archivo.');

    close(mae);
end;

var
    mae: fileMaestro;
    input:integer;
begin

    assing(mae, 'maestro');
    writeln('ingrese el anio del informe que desea ver');
    readln(input)
    imprimir(mae,input);
    
end;