program generarArchivoAves;

type
    aves = record
        code: integer;
        nombre: string;
        familia: string;
        descripcion: string;
        zona: string;
    end;

    fileMaestro = file of aves;

var
    mae: fileMaestro;
    reg: aves;

procedure agregarAve(codigo: integer; nom, fam, desc, zonaGeo: string);
begin
    reg.code := codigo;
    reg.nombre := nom;
    reg.familia := fam;
    reg.descripcion := desc;
    reg.zona := zonaGeo;
    write(mae, reg);
end;

begin
    assign(mae, 'archivo_aves');
    rewrite(mae);

    agregarAve(101, 'Cóndor Andino', 'Cathartidae', 'Ave carroñera de gran tamaño', 'Cordillera de los Andes');
    agregarAve(202, 'Águila Harpía', 'Accipitridae', 'Ave rapaz poderosa', 'Selvas tropicales de América');
    agregarAve(303, 'Guacamayo Azul', 'Psittacidae', 'Loro de plumaje azul brillante', 'Amazonas');
    agregarAve(404, 'Pato Vapor', 'Anatidae', 'Ave acuática no voladora', 'Patagonia');
    agregarAve(505, 'Tucán Pico Iris', 'Ramphastidae', 'Ave colorida de gran pico', 'Centroamérica');

    close(mae);
    writeln('Archivo "archivo_aves" generado con éxito.');
end.

program generarArchivoAves;

type
    aves = record
        code: integer;
        nombre: string;
        familia: string;
        descripcion: string;
        zona: string;
    end;

    fileMaestro = file of aves;

var
    mae: fileMaestro;
    reg: aves;

procedure agregarAve(codigo: integer; nom, fam, desc, zonaGeo: string);
begin
    reg.code := codigo;
    reg.nombre := nom;
    reg.familia := fam;
    reg.descripcion := desc;
    reg.zona := zonaGeo;
    write(mae, reg);
end;

begin
    assign(mae, 'archivo_aves');
    rewrite(mae);

    agregarAve(101, 'Cóndor Andino', 'Cathartidae', 'Ave carroñera de gran tamaño', 'Cordillera de los Andes');
    agregarAve(202, 'Águila Harpía', 'Accipitridae', 'Ave rapaz poderosa', 'Selvas tropicales de América');
    agregarAve(303, 'Guacamayo Azul', 'Psittacidae', 'Loro de plumaje azul brillante', 'Amazonas');
    agregarAve(404, 'Pato Vapor', 'Anatidae', 'Ave acuática no voladora', 'Patagonia');
    agregarAve(505, 'Tucán Pico Iris', 'Ramphastidae', 'Ave colorida de gran pico', 'Centroamérica');

    close(mae);
    writeln('Archivo "archivo_aves" generado con éxito.');
end.