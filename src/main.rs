use std::{borrow::Borrow, fs::{create_dir_all, File}};
use rand::Rng;
use serde::Deserialize;
use rand::seq::SliceRandom;
use std::io::Write;
use fake::{faker::chrono::en::{Date}, Fake};

#[derive(Deserialize, Debug)]
struct Regione {
	codice: String,
	nome: String
}

#[derive(Deserialize, Debug)]
struct RegioneProvincia {
	codice_regione: String,
	sigla_provincia: String
}

#[derive(Debug)]
struct Provincia {
	sigla: String,
	regione: String,
	nome: String
}

#[derive(Deserialize, Debug)]
struct ProvinciaTemp {
	sigla: String,
	nome: String
}

#[derive(Deserialize, Debug)]
struct ProvinciaComune {
	sigla_provincia: String,
	codice_comune: String
}

#[derive(Debug)]
struct Comune {
	codice: String,
	provincia: String,
	nome: String
}

#[derive(Deserialize, Debug)]
struct ComuneTemp {
	codice: String,
	nome: String
}

#[derive(Debug)]
struct ComuneCasello {
	codice_comune: String,
	codice_casello: u32,
	cod_naz: String
}

#[derive(Debug)]
struct Casello {
	codice: u32,
	codice_comune: String,
	nome: String,
	x: f64,
	y: f64,
	cod_naz: String,
	is_automatico: bool,
	data_automazione: String,
}

#[derive(Debug)]
struct Audostrada {
	cod_naz: String,
	cod_eu: String,
	nome: String,
	lunghezza: u32,
}


fn main() {

	// REGIONI
	let _regioni_list:Vec<Regione> = fill_regioni();
	
	// REGIONIPROVINCIE
	let _regioni_provincie_list:Vec<RegioneProvincia> = fill_regioni_provincie();
	
	// PROVINCIE
	let provincie_list:Vec<Provincia> = fill_provincie();
	
	// PROVINCIECOMUNI
	let _comuni_provincie_list:Vec<ProvinciaComune> = fill_provincie_comuni();
	
	// COMUNI
	let comuni_list:Vec<Comune> = fill_comuni();

	// AUTOSTRADE
	let autostrade_list:Vec<Audostrada> = fill_autostrade();

	// CASELLO
	let caselli_list:Vec<Casello> = fill_caselli(autostrade_list.borrow(), comuni_list.borrow());
	
	// COMUNECASELLO
	let comuni_caselli_list:Vec<ComuneCasello> = fill_comuni_caselli(&comuni_list, &caselli_list);

	// Creo cartella di output
	let _ = create_dir_all("./output");

	//
	// Scrittura delle tabelle in formato CSV
	//

	// scrittura di caselli_list
	let mut caselli_writer = File::create("./output/caselli_list.csv").unwrap();
	writeln!(caselli_writer, "codice,nome,x,y,cod_naz,is_automatico,data_automazione").unwrap();
	// println!("\nStampa di caselli_list");
	for record in caselli_list {
		writeln!(caselli_writer, "{},{},{},{},{},{},{}", record.codice, record.nome, record.x, record.y, record.cod_naz, record.is_automatico, record.data_automazione).unwrap();
	}

	// Scrittura autostrade_list
	let mut autostrade_writer = File::create("./output/autostrade_list.csv").unwrap();
	writeln!(autostrade_writer, "cod_naz,cod_eu,nome,lunghezza").unwrap();
	// println!("\nStampa di autostrade_list");
	for record in autostrade_list {
		writeln!(autostrade_writer, "{},{},{},{}", record.cod_naz, record.cod_eu, record.nome, record.lunghezza).unwrap();
	}

	// Scrittura provincie_list
	let mut provincie_writer = File::create("./output/provincie_list.csv").unwrap();
	writeln!(provincie_writer, "sigla,regione,nome").unwrap();
	for record in provincie_list {
		writeln!(provincie_writer, "{},{},{}", record.sigla, record.regione, record.nome).unwrap();
	}

	// scrittura di comuni_list
	let mut comuni_writer = File::create("./output/comuni_list.csv").unwrap();
	writeln!(comuni_writer, "codice,provincia,nome").unwrap();
	// println!("\nStampa di comuni_list");
	for record in comuni_list {
		writeln!(comuni_writer, "{},{},{}", record.codice, record.provincia, record.nome).unwrap();
	}
	

}

fn fill_regioni() -> Vec<Regione> {
	let mut regioni_csv = csv::Reader::from_path("./gi_regioni.csv").unwrap();
	let mut regioni_list:Vec<Regione> = vec![];

	// println!("\nStampa di Regione");

	for result in regioni_csv.deserialize() {
		let record: Regione = result.unwrap();
		// println!("{},{}", record.codice, record.nome);
		regioni_list.push(record);
	}

	regioni_list
}

fn fill_regioni_provincie() -> Vec<RegioneProvincia> {
	let mut regioni_csv = csv::Reader::from_path("./gi_regioni-province.csv").unwrap();
	let mut regioni_provincie_list:Vec<RegioneProvincia> = vec![];

	// println!("\nStampa di RegioneProvincia");

	for result in regioni_csv.deserialize() {
		let record: RegioneProvincia = result.unwrap();
		// println!("{},{}", record.codice_regione, record.sigla_provincia);
		regioni_provincie_list.push(record);
	}

	regioni_provincie_list
}

fn fill_provincie() -> Vec<Provincia> {

	let mut regioni_list:Vec<Regione> = fill_regioni();

	let regioni_provincie_list:Vec<RegioneProvincia> = fill_regioni_provincie();

	let mut provincie_csv = csv::Reader::from_path("./gi_province.csv").unwrap();
	let mut provincie_temp_list:Vec<ProvinciaTemp> = vec![];
	let mut provincie_list:Vec<Provincia> = vec![];

	// println!("\nStampa di Provincia");

	for result in provincie_csv.deserialize() {
		let record: ProvinciaTemp = result.unwrap();
		// println!("{},{}", record.sigla, record.nome);
		provincie_temp_list.push(record);
	}

	for provincia_temp in provincie_temp_list.as_slice() {
		println!("{:?}", provincia_temp);
		for regione_provincia in regioni_provincie_list.as_slice() {
			for regione in regioni_list.as_slice() {
				if regione.codice == regione_provincia.codice_regione && regione_provincia.sigla_provincia == provincia_temp.sigla {
					let provincia_element:Provincia = Provincia {
						sigla: regione_provincia.sigla_provincia.clone(),
						regione: regione_provincia.codice_regione.clone(),
						nome: provincia_temp.nome.clone(),
					};
					provincie_list.push(provincia_element);
				}
			}
		}
	}

	provincie_list
}

fn fill_provincie_comuni() -> Vec<ProvinciaComune> {

	let mut regioni_csv = csv::Reader::from_path("./gi_province-comuni.csv").unwrap();
	let mut comuni_provincie_list:Vec<ProvinciaComune> = vec![];

	// println!("\nStampa di ProvinciaComune");

	for result in regioni_csv.deserialize() {
		let record: ProvinciaComune = result.unwrap();
		// println!("{},{}", record.sigla_provincia, record.codice_comune);
		comuni_provincie_list.push(record);
	}

	comuni_provincie_list
}

// Funzione generazione comuni
fn fill_comuni() -> Vec<Comune> {

	let provincie_list:Vec<Provincia> = fill_provincie();
	let comuni_provincie_list:Vec<ProvinciaComune> = fill_provincie_comuni();


	let mut comuni_csv = csv::Reader::from_path("./gi_comuni.csv").unwrap();
	let mut comuni_list_temp:Vec<ComuneTemp> = vec![];
	let mut comuni_list:Vec<Comune> = vec![];

	// println!("\nStampa di Comune");
	for result in comuni_csv.deserialize() {
		let record: ComuneTemp = result.unwrap();
		// println!("{},{}", record.codice, record.nome);
		comuni_list_temp.push(record);
	}

	for provincia in provincie_list.as_slice() {
		println!("{:?}", provincia);
		for comune_temp in comuni_list_temp.as_slice() {
			for comune_provincia in comuni_provincie_list.as_slice() {
				if provincia.sigla == comune_provincia.sigla_provincia && comune_provincia.codice_comune == comune_temp.codice {
					let comune_element:Comune = Comune {
						codice: comune_provincia.codice_comune.clone(),
						provincia: provincia.sigla.clone(),
						nome: comune_temp.nome.clone(),
					};
					comuni_list.push(comune_element);
				}
			}
		}
	}

	comuni_list
}

// Funzione di generazione dati autostrada
fn fill_autostrade() -> Vec<Audostrada> {

	let mut rng = rand::thread_rng();
	
	let mut autostrade_list:Vec<Audostrada> = vec![];

	// Genera dati
	for element in 1..=100 {
		let record: Audostrada = Audostrada {
			cod_naz: format!("A{}", element),
			cod_eu: format!("E{}", element),
			nome: format!("A{}-E{}", element, element),
			lunghezza: rng.gen()
		};

		autostrade_list.push(record);
	}
	autostrade_list
}

// Funzione di generazione dati autostrada
fn fill_caselli(autostrade_list:&Vec<Audostrada>, comuni_list:&Vec<Comune>) -> Vec<Casello> {

	let mut rng = rand::thread_rng();
	
	let mut caselli_list:Vec<Casello> = vec![];

	// Genera dati
	for element in 1..=1000 {
		let mut record: Casello = Casello {
			codice: element,
			nome: format!("{}", element),
			codice_comune: comuni_list.choose(&mut rng).unwrap().codice.clone(),
			x: 10000.0 * (rng.gen::<f64>() - 0.5),
			y: 10000.0 * (rng.gen::<f64>() - 0.5),
			cod_naz: autostrade_list.choose(&mut rng).unwrap().cod_naz.clone(),
			is_automatico: rng.gen_bool(0.5),
			data_automazione: "".to_string(),
		};

		if record.is_automatico {
			let data:String = Date().fake();
			record.data_automazione = format!("{:?}", data);
		}

		caselli_list.push(record);
	}
	caselli_list
}

fn fill_comuni_caselli(comuni_list:&Vec<Comune>, caselli_list:&Vec<Casello>) -> Vec<ComuneCasello> {
	let mut rng = rand::thread_rng();
	
	let mut comuni_caselli_list:Vec<ComuneCasello> = vec![];

	for _ in 1..=10000 {
		let record: ComuneCasello = ComuneCasello {
			cod_naz: caselli_list.choose(&mut rng).unwrap().cod_naz.clone(),
			codice_casello: caselli_list.choose(&mut rng).unwrap().codice,
			codice_comune: comuni_list.choose(&mut rng).unwrap().codice.clone(),
		};

		comuni_caselli_list.push(record);
	}
	comuni_caselli_list
}