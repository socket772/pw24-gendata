use std::fs::read_to_string;

struct Comune {
	codice: u16,
	nome: String
}

fn main() {

	let comuni:Vec<Comune> = riempi_comune();
	// Print di debug
	// for elemento in comuni {
	// 	println!("{}, {}", elemento.codice, elemento.nome)
	// }

	
}

fn riempi_comune() -> Vec<Comune> {

	let mut comuni:Vec<Comune> = vec![];

	let comune_nome_temp:Vec<String> = read_to_string("./gi_comuni_edit.txt")
		.unwrap() // Panica se ci sono errori di lettura
		.lines() // Dividi la stringa in un iteratore
		.map(String::from) // Trasforma le linee in stringhe
		.collect(); // trasforma tutte le stringhe in una collezione

	let mut i:u16 = 0;
	for citta in comune_nome_temp {
		i = i + 1;
		comuni.push(
			Comune {
				codice: i,
				nome: citta
		});
	}

	return comuni;
}