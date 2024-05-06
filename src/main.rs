
use std::borrow::Borrow;

use geo::Coord;
use rand::Rng;
use serde::Deserialize;
use rand::seq::SliceRandom;

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

}

struct Casello {
	codice: u32,
	nome: String,
	coordinate: Coord,
	cod_naz_autostrada: String,
	
}

struct Audostrada {
	cod_naz: String,
	cod_eu: String,
	nome: String,
	lunghezza: u32,
}

struct CaselloAutostrada {
	cod_naz: String,
	cod_casello: u32,
}

fn main() {

	// REGIONI
	let regioni_list:Vec<Regione> = fill_regioni();
	
	// REGIONIPROVINCIE
	let regioni_provincie_list:Vec<RegioneProvincia> = fill_regioni_provincie();
	
	// PROVINCIE
	let provincie_list:Vec<Provincia> = fill_provincie();
	
	// PROVINCIECOMUNI
	let comuni_provincie_list:Vec<ProvinciaComune> = fill_provincie_regioni();
	
	// COMUNI
	let comuni_list:Vec<Comune> = fill_comuni();

	// AUTOSTRADE
	let autostrade_list:Vec<Audostrada> = fill_autostrade();

	// CASELLO
	let casello_list:Vec<Casello> = fill_casello(autostrade_list.borrow());
	
	// CASELLOAUTOSTRADA
	let casello_autostrada_list:Vec<CaselloAutostrada> = fill_casello_autostrada(casello_list.borrow(), autostrade_list.borrow());

	// Scrittura delle tabelle in formato CSV

}

fn fill_regioni() -> Vec<Regione> {
	let mut regioni_csv = csv::Reader::from_path("./gi_regioni.csv").unwrap();
	let mut regioni_list:Vec<Regione> = vec![];

	println!("\nStampa di Regione");

	for result in regioni_csv.deserialize() {
		let record: Regione = result.unwrap();
		println!("{},{}", record.codice, record.nome);
		regioni_list.push(record);
	}

	regioni_list
}

fn fill_regioni_provincie() -> Vec<RegioneProvincia> {
	let mut regioni_csv = csv::Reader::from_path("./gi_regioni-province.csv").unwrap();
	let mut regioni_provincie_list:Vec<RegioneProvincia> = vec![];

	println!("\nStampa di RegioneProvincia");

	for result in regioni_csv.deserialize() {
		let record: RegioneProvincia = result.unwrap();
		println!("{},{}", record.codice_regione, record.sigla_provincia);
		regioni_provincie_list.push(record);
	}

	regioni_provincie_list
}

fn fill_provincie() -> Vec<Provincia> {
	let mut provincie_csv = csv::Reader::from_path("./gi_province.csv").unwrap();
	let mut provincie_list:Vec<Provincia> = vec![];

	println!("\nStampa di Provincia");

	for result in provincie_csv.deserialize() {
		let record: Provincia = result.unwrap();
		println!("{},{}", record.sigla, record.nome);
		provincie_list.push(record);
	}

	provincie_list
}

fn fill_provincie_regioni() -> Vec<ProvinciaComune> {
	let mut regioni_csv = csv::Reader::from_path("./gi_province-comuni.csv").unwrap();
	let mut comuni_provincie_list:Vec<ProvinciaComune> = vec![];

	println!("\nStampa di ProvinciaComune");

	for result in regioni_csv.deserialize() {
		let record: ProvinciaComune = result.unwrap();
		println!("{},{}", record.sigla_provincia, record.codice_comune);
		comuni_provincie_list.push(record);
	}

	comuni_provincie_list
}

// Funzione generazione comuni
fn fill_comuni() -> Vec<Comune> {
	let mut comuni_csv = csv::Reader::from_path("./gi_comuni.csv").unwrap();
	let mut comuni_list:Vec<Comune> = vec![];

	println!("\nStampa di Comune");

	for result in comuni_csv.deserialize() {
		let record: Comune = result.unwrap();
		println!("{},{}", record.codice, record.nome);
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

	// Stampa lista
	println!("\nStampa di Autostrada");
	for record in autostrade_list.as_slice() {
		println!("{},{},{},{}", record.cod_naz, record.cod_eu, record.nome, record.lunghezza);
	}
	
	autostrade_list
}

// Funzione di generazione dati autostrada
fn fill_casello(autostrade_list:&Vec<Audostrada>) -> Vec<Casello> {

	let mut rng = rand::thread_rng();
	
	let mut caselli_list:Vec<Casello> = vec![];

	// Genera dati
	for element in 1..=1000 {
		let record: Casello = Casello {
			codice: element,
			nome: format!("{}", element),
			coordinate: Coord { x: rng.gen(), y: rng.gen() },
			cod_naz_autostrada: autostrade_list.choose(&mut rng).unwrap().cod_naz.clone(),
		};

		caselli_list.push(record);
	}

	// Stampa lista
	println!("\nStampa di Autostrada");
	for record in caselli_list.as_slice() {
		println!("{},{}", record.cod_naz_autostrada, record.codice);
	}
	
	caselli_list
}

fn fill_casello_autostrada(casello_list:&Vec<Casello>, autostrade_list:&Vec<Audostrada>) -> Vec<CaselloAutostrada> {
	let mut rng = rand::thread_rng();
	
	let mut casello_autostrada_list:Vec<CaselloAutostrada> = vec![];

	for element in 1..=10000 {
		let record: CaselloAutostrada = CaselloAutostrada {
			cod_naz: autostrade_list.choose(&mut rng).unwrap().cod_naz.clone(),
			cod_casello: casello_list.choose(&mut rng).unwrap().codice,
		};

		casello_autostrada_list.push(record);
	}

	println!("\nStampa di Autostrada");
	for record in casello_autostrada_list.as_slice() {
		println!("{},{}", record.cod_casello, record.cod_naz);
	}

	casello_autostrada_list
}