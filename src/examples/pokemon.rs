use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};

use std::fs::File;

#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    let func = handler_fn(pokemon_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn pokemon_handler(event: Request, ctx: Context) -> Result<Response, Error> {
    let command = event.command;
    let matching_name = get_matching_initial(&command, pokemon_list());
    let resp = Response {
        req_id: ctx.request_id,
        msg: format!("Your pokemon name is: {}.", matching_name),
    };
    Ok(resp)
}

pub fn get_matching_initial(initial: &str, pmon_list: Vec<String>) -> String {
    match pmon_list.into_iter().find(
            |x|str::to_uppercase(initial).chars().next().unwrap() == x.chars().next().unwrap()) {
        Some(result) => result,
        None => "No pokemon matches that initial".to_string(),
    }
}

pub fn load_pokemon_data() -> serde_json::Value {
    let file = std::fs::File::open("src/pokemon_data.json").expect("file should open read only");
    serde_json::from_reader(file).expect("JSON not formatted")
}

// get full list from json
pub fn pokemon_list_from_json() -> Vec<String> {
    let list = load_pokemon_data();
    fn prepare_for_list(pokemon_name: &serde_json::Value) -> String{
        let mut string_name = pokemon_name.to_string();
        string_name.remove(0);
        string_name.pop();
        string_name
    }
    let mut counter = 0;
    let mut pokemon_vec = vec![];
    while let Some(list_item) = list.get(counter) {
        pokemon_vec.push(prepare_for_list(list_item));
        counter +=1;
    }
    pokemon_vec
}

#[cfg(test)]
mod pokemon {
    #[test]
    fn test_pokemon_list() {
        let p_list = pokemon_list();
        let first_item = p_list.get(0).unwrap();
        assert_eq!("Bulbasaur", first_item);
    }
    #[test]
    fn test_get_matching_initial_lowercase() {
        let p_list = pokemon_list();
        let matching_pokemon = get_matching_initial("b", p_list);
        assert_eq!("Bulbasaur", matching_pokemon);
    }
    #[test]
    fn test_get_matching_initial_uppercase() {
        let p_list = pokemon_list();
        let matching_pokemon = get_matching_initial("C", p_list);
        assert_eq!("Charmander", matching_pokemon);
    }
    #[test]
    fn test_get_matching_initial_none() {
        let p_list = pokemon_list();
        let matching_pokemon = get_matching_initial("_", p_list);
        assert_eq!("No pokemon matches that initial", matching_pokemon);
    }
    #[test]
    fn test_load_pokemon_data() {
        let json_file_loaded = load_pokemon_data();
        let first_item = json_file_loaded.get(0).unwrap() ;
        assert_eq!("Bulbasaur", first_item);
    }
}


// return short list
pub fn pokemon_list() -> Vec<String> {
	let pokemon_list = vec![
		"Bulbasaur", "Ivysaur", "Venusaur", "Charmander", "Charmeleon", "Charizard", "Squirtle",
		"Wartortle", "Blastoise", "Caterpie", "Metapod", "Butterfree", "Weedle", "Kakuna",
		"Beedrill", "Pidgey", "Pidgeotto", "Pidgeot", "Rattata", "Raticate", "Spearow", "Fearow",
		"Ekans", "Arbok", "Pikachu", "Raichu", "Sandshrew", "Sandslash", "Nidoran♀", "Nidorina",
		"Nidoqueen", "Nidoran♂", "Nidorino", "Nidoking", "Clefairy", "Clefable", "Vulpix", "Ninetales",
		"Jigglypuff", "Wigglytuff", "Zubat", "Golbat", "Oddish", "Gloom", "Vileplume", "Paras",
		"Parasect", "Venonat", "Venomoth", "Diglett", "Dugtrio", "Meowth", "Persian", "Psyduck", "Golduck", "Mankey", "Primeape", "Growlithe",
		"Arcanine", "Poliwag", "Poliwhirl", "Poliwrath", "Abra", "Kadabra", "Alakazam", "Machop", "Machoke", "Machamp", "Bellsprout",
		"Weepinbell", "Victreebel", "Tentacool", "Tentacruel", "Geodude", "Graveler", "Golem", "Ponyta", "Rapidash", "Slowpoke", "Slowbro",
		"Magnemite", "Magneton", "Farfetch’d", "Doduo", "Dodrio", "Seel", "Dewgong", "Grimer", "Muk", "Shellder", "Cloyster",
		"Gastly", "Haunter", "Gengar", "Onix", "Drowzee", "Hypno", "Krabby", "Kingler", "Voltorb", "Electrode",
		"Exeggcute", "Exeggutor", "Cubone", "Marowak", "Hitmonlee", "Hitmonchan", "Lickitung", "Koffing", "Weezing", "Rhyhorn",
		"Rhydon", "Chansey", "Tangela", "Kangaskhan", "Horsea", "Seadra", "Goldeen", "Seaking", "Staryu", "Starmie", "Mr. Mime",
		"Scyther", "Jynx", "Electabuzz", "Magmar", "Pinsir", "Tauros", "Magikarp", "Gyarados", "Lapras", "Ditto",
		"Eevee", "Vaporeon", "Jolteon", "Flareon", "Porygon", "Omanyte", "Omastar", "Kabuto", "Kabutops", "Aerodactyl",
		"Snorlax", "Articuno", "Zapdos", "Moltres", "Dratini", "Dragonair", "Dragonite", "Mewtwo", "Mew", "Chikorita",
		"Bayleef", "Meganium", "Cyndaquil", "Quilava", "Typhlosion", "Totodile", "Croconaw", "Feraligatr", "Sentret", "Furret",
		"Hoothoot", "Noctowl", "Ledyba", "Ledian", "Spinarak", "Ariados", "Crobat", "Chinchou", "Lanturn", "Pichu",
		"Cleffa", "Igglybuff", "Togepi", "Togetic", "Natu", "Xatu", "Mareep", "Flaaffy", "Ampharos", "Bellossom",
		"Marill", "Azumarill", "Sudowoodo", "Politoed", "Hoppip", "Skiploom", "Jumpluff", "Aipom", "Sunkern", "Sunflora",
		"Yanma", "Wooper", "Quagsire", "Espeon", "Umbreon", "Murkrow", "Slowking", "Misdreavus", "Unown", "Wobbuffet",
		"Girafarig", "Pineco", "Forretress", "Dunsparce", "Gligar", "Steelix", "Snubbull", "Granbull", "Qwilfish", "Scizor",
		"Shuckle", "Heracross", "Sneasel", "Teddiursa", "Ursaring", "Slugma", "Magcargo", "Swinub", "Piloswine", "Corsola",
		"Remoraid", "Octillery", "Delibird", "Mantine", "Skarmory", "Houndour", "Houndoom", "Kingdra", "Phanpy", "Donphan",
		"Porygon2", "Stantler", "Smeargle", "Tyrogue", "Hitmontop", "Smoochum", "Elekid", "Magby", "Miltank", "Blissey",
		"Raikou", "Entei", "Suicune", "Larvitar", "Pupitar", "Tyranitar", "Lugia", "Ho-Oh", "Celebi", "Treecko",
		"Grovyle", "Sceptile", "Torchic", "Combusken", "Blaziken", "Mudkip", "Marshtomp", "Swampert", "Poochyena", "Mightyena",
		"Zigzagoon", "Linoone", "Wurmple", "Silcoon", "Beautifly", "Cascoon", "Dustox", "Lotad", "Lombre", "Ludicolo",
		"Seedot", "Nuzleaf", "Shiftry", "Taillow", "Swellow", "Wingull", "Pelipper", "Ralts", "Kirlia", "Gardevoir",
		"Surskit", "Masquerain", "Shroomish", "Breloom", "Slakoth", "Vigoroth", "Slaking", "Nincada", "Ninjask", "Shedinja",
		"Whismur", "Loudred", "Exploud", "Makuhita", "Hariyama", "Azurill", "Nosepass", "Skitty", "Delcatty", "Sableye",
		"Regidrago", "Glastrier", "Spectrier", "Calyrex"
	];

	pokemon_list.iter().map(|x|x.to_string()).collect()
}