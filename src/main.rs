use std::{borrow::Borrow, fs::{create_dir_all, File}};
use rand::Rng;
use serde::Deserialize;
use rand::seq::SliceRandom;
use std::io::Write;
use fake::{faker::chrono::en::{Date}, Fake};

#[derive(Deserialize)]
struct Regione {
	codice: String,
	nome: String
}

#[derive(Deserialize)]
struct RegioneProvincia {
	codice_regione: String,
	sigla_provincia: String
}

#[derive(Deserialize)]
struct Provincia {
	sigla: String,
	nome: String
}

#[derive(Deserialize)]
struct ProvinciaComune {
	sigla_provincia: String,
	codice_comune: String
}

#[derive(Deserialize)]
struct Comune {
	codice: String,
	nome: String
}

struct ComuneCasello {
	codice_comune: String,
	codice_casello: u32,
	cod_naz: String
}

struct Casello {
	codice: u32,
	nome: String,
	x: f64,
	y: f64,
	cod_naz: String,
	is_automatico: bool,
	data_automazione: String,
}

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
	let _provincie_list:Vec<Provincia> = fill_provincie();
	
	// PROVINCIECOMUNI
	let _comuni_provincie_list:Vec<ProvinciaComune> = fill_provincie_regioni();
	
	// COMUNI
	let comuni_list:Vec<Comune> = fill_comuni();

	// AUTOSTRADE
	let autostrade_list:Vec<Audostrada> = fill_autostrade();

	// CASELLO
	let caselli_list:Vec<Casello> = fill_caselli(autostrade_list.borrow());
	
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
		println!("{},{},{},{},{},{},{}", record.codice, record.nome, record.x, record.y, record.cod_naz, record.is_automatico, record.data_automazione);
		writeln!(caselli_writer, "{},{},{},{},{},{},{}", record.codice, record.nome, record.x, record.y, record.cod_naz, record.is_automatico, record.data_automazione).unwrap();
	}

	// Scrittura autostrade_list
	let mut autostrade_writer = File::create("./output/autostrade_list.csv").unwrap();
	writeln!(autostrade_writer, "cod_naz,cod_eu,nome,lunghezza").unwrap();
	
	// println!("\nStampa di autostrade_list");
	for record in autostrade_list {
		println!("{},{},{},{}", record.cod_naz, record.cod_eu, record.nome, record.lunghezza);
		writeln!(autostrade_writer, "{},{},{},{}", record.cod_naz, record.cod_eu, record.nome, record.lunghezza).unwrap();
	}

	// scrittura di comuni_caselli_list
	let mut comuni_caselli_writer = File::create("./output/comuni_caselli.csv").unwrap();
	writeln!(comuni_caselli_writer, "codice_comune,codice_casello,cod_naz").unwrap();
	
	// println!("\nStampa di comuni_caselli_list");
	for record in comuni_caselli_list {
		println!("{},{},{}", record.codice_casello, record.cod_naz, record.codice_comune);
		writeln!(comuni_caselli_writer, "{},{},{}", record.codice_comune, record.codice_casello, record.cod_naz).unwrap();
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
	let mut provincie_csv = csv::Reader::from_path("./gi_province.csv").unwrap();
	let mut provincie_list:Vec<Provincia> = vec![];

	// println!("\nStampa di Provincia");

	for result in provincie_csv.deserialize() {
		let record: Provincia = result.unwrap();
		// println!("{},{}", record.sigla, record.nome);
		provincie_list.push(record);
	}

	provincie_list
}

fn fill_provincie_regioni() -> Vec<ProvinciaComune> {
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
	let mut comuni_csv = csv::Reader::from_path("./gi_comuni.csv").unwrap();
	let mut comuni_list:Vec<Comune> = vec![];

	// println!("\nStampa di Comune");
	for result in comuni_csv.deserialize() {
		let record: Comune = result.unwrap();
		// println!("{},{}", record.codice, record.nome);
		comuni_list.push(record);
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
fn fill_caselli(autostrade_list:&Vec<Audostrada>) -> Vec<Casello> {

	let mut rng = rand::thread_rng();
	
	let mut caselli_list:Vec<Casello> = vec![];

	// Genera dati
	for element in 1..=1000 {
		let mut record: Casello = Casello {
			codice: element,
			nome: format!("{}", element),
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
			codice_casello: caselli_list.choose(&mut rng).unwrap().codice.clone(),
			codice_comune: comuni_list.choose(&mut rng).unwrap().codice.clone(),
		};

		comuni_caselli_list.push(record);
	}
	comuni_caselli_list
}