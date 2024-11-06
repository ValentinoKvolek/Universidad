program parcial6;
type
	subrMes = 0..12;
	
	subrDiagnostico = 1..6;
	
	atencion = record
		dni:integer;
		mes:subrMes;
		codeDiagnostico:subrDiagnostico;
	end;
	
	vectorContador = array [subrDiagnostico] of integer;
	
	arbol = ^nodo;
	
	nodo = record
		dni:integer;
		cantTotal:integer;
		hI:arbol;
		hD:arbol;
	end;

	
	procedure generarEstructuras(var a:arbol; var vc:vectorContador);
		
		procedure leerAtencion(var elem:atencion);
		begin
			writeln('ingrese mes: ');
			elem.mes:=random(12);
			writeln;
			writeln(elem.mes);
			writeln;
			if (elem.mes <> 0) then begin
				writeln('ingrese dni');
				elem.dni:=random(10) +1;
				writeln;
				writeln(elem.dni);
				writeln;
				writeln('ingrese codigo de diagnostico: ');
				elem.codeDiagnostico:=random(5)+1;
				writeln;
				writeln(elem.codeDiagnostico);
				writeln;
			end;
		end;
		
		procedure inicializar(var vc:vectorContador);
		var
		i:integer;
		begin
			for i:=1 to 6 do 
				vc[i]:=0;
		end;
		
		procedure llenarArbol(var a:arbol; elem:atencion);
		begin
			if(a = nil) then begin
				new(a); a^.dni:=elem.dni; a^.cantTotal:=1; a^.hI:=nil; a^.hD:=nil;
			end
			else if(elem.dni = a^.dni) then a^.cantTotal:= a^.cantTotal + 1
			else if(elem.dni < a^.dni) then llenarArbol(a^.hI,elem)
			else llenarArbol(a^.hD,elem);
		end;
		
		
		procedure llenarVector(var vc:vectorContador; elem:atencion);
		begin
			vc[elem.codeDiagnostico]:=vc[elem.codeDiagnostico] +1;
		end;
		
	var
	elem:atencion;
	begin
		leerAtencion(elem);
		inicializar(vc);
		while(elem.mes<>0) do begin
			llenarArbol(a,elem);
			llenarVector(vc,elem);
			leerAtencion(elem);
		end;
	end;
	
	
	procedure incisoB(a:arbol);
	
		procedure contarPacientes(a:arbol; min:integer; max:integer; valor:integer; var cant:integer);
		begin
			if(a<>nil) then begin
				if (a^.dni > min) then begin
					if(a^.dni<max) then begin
						if (a^.cantTotal > valor) then 
							 cant:=cant+1;
						contarPacientes(a^.hI,min,max,valor,cant);
						contarPacientes(a^.hD,min,max,valor,cant);
					end
					else
						contarPacientes(a^.hI,min,max,valor,cant);
				end
				else
					contarPacientes(a^.hD,min,max,valor,cant);
			end;
		end;
	var
	dni1:integer;
	dni2:integer;
	valor:integer;
	max:integer;
	min:integer;
	cant:integer;
	begin
		cant:=0;
		writeln('ingrese le primer dni:');
		readln(dni1);
		writeln;
		writeln('ingrese le segundo dni:');
		readln(dni2);
		writeln;
		writeln('ingrese valor:');
		readln(valor);
		writeln;
		if(dni1>dni2) then begin
			max:=dni1;
			min:=dni2;
		end
		else begin
			max:=dni2;
			min:=dni1;
		end;
		contarPacientes(a,min,max,valor,cant);
		writeln('la cantidad de pacientes fueron: ', cant);
	end;
	
	procedure incisoC(vc:vectorContador);
	
		procedure recorrer(vc:vectorContador; var cant:integer; var pos:integer);
		begin
			if(pos<=6) then begin
				pos:=pos+1;
				if (vc[pos] =  0) then 
					cant:=cant + 1;
				recorrer(vc,cant,pos);
			end;
		end;
		
	var
	pos:integer;
	cant:integer;
	begin
		cant:=0;
		pos:=0;
		recorrer(vc,cant,pos);
		writeln;
		writeln('cantidad de diagnosticos en 0 fueron: ', cant);
	end;
	
	
	
var
a:arbol;
vc:vectorContador;
begin
	a:=nil;
	randomize;
	generarEstructuras(a,vc);
	incisoB(a);
	incisoC(vc);
end.
