use std::vec;

use serde::Deserialize;

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

struct Audostrada {
	cod_naz: u8,
	cod_eu: u8
	
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