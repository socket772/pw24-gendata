use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Regione {
	codice: String,
	nome: String
}

#[derive(Debug, Deserialize)]
struct RegioneProvincia {
	codice_regione: String,
	sigla_provincia: String
}

#[derive(Debug, Deserialize)]
struct Provincia {
	sigla: String,
	nome: String
}

#[derive(Debug, Deserialize)]
struct ProvinciaComune {
	sigla_provincia: String,
	codice_comune: String
}

#[derive(Debug, Deserialize)]
struct Comune {
	codice: String,
	nome: String
}



fn main() {

	// REGIONI
	let mut regioni_csv = csv::Reader::from_path("./gi_regioni.csv").unwrap();
	let mut regioni_list:Vec<Regione> = vec![];

	for result in regioni_csv.deserialize() {
		let record: Regione = result.unwrap();
		println!("{:?}", record);
		regioni_list.push(record);
	}

	// REGIONIPROVINCIE
	let mut regioni_csv = csv::Reader::from_path("./gi_regioni-province.csv").unwrap();
	let mut regioni_provincie_list:Vec<RegioneProvincia> = vec![];

	for result in regioni_csv.deserialize() {
		let record: RegioneProvincia = result.unwrap();
		println!("{:?}", record);
		regioni_provincie_list.push(record);
	}

	// PROVINCIE
	let mut provincie_csv = csv::Reader::from_path("./gi_province.csv").unwrap();
	let mut provincie_list:Vec<Provincia> = vec![];

	for result in provincie_csv.deserialize() {
		let record: Provincia = result.unwrap();
		println!("{:?}", record);
		provincie_list.push(record);
	}

	// REGIONIPROVINCIE
	let mut regioni_csv = csv::Reader::from_path("./gi_comuni-province.csv").unwrap();
	let mut comuni_provincie_list:Vec<ProvinciaComune> = vec![];

	for result in regioni_csv.deserialize() {
		let record: ProvinciaComune = result.unwrap();
		println!("{:?}", record);
		comuni_provincie_list.push(record);
	}

	// COMUNI
	let mut comuni_csv = csv::Reader::from_path("./gi_comuni.csv").unwrap();
	let mut comuni_list:Vec<Comune> = vec![];

	for result in comuni_csv.deserialize() {
		let record: Comune = result.unwrap();
		println!("{:?}", record);
		comuni_list.push(record);
	}
	
}