// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct Names<'a> {
    names_vector: Vec<&'a str>,
}

struct Titles<'a> {
    titles_vector: Vec<&'a str>,
}

fn zip_in<'a>(data: &Vec<&'a str>, names: &mut Names<'a>, titles: &mut Titles<'a>) {
    names.names_vector = data.iter().filter_map(|x| x.split(',').nth(1)).collect();

    titles.titles_vector = data.iter().filter_map(|x| x.split(',').nth(4)).collect();

    for (name, title) in names.names_vector.iter().zip(titles.titles_vector.iter()) {
        println!("Name: {:?}, Title: {:?}", name, title.trim());
    }
}

fn main() {
    let data: Vec<&str> = MOCK_DATA.split('\n').skip(1).collect();
    let mut names = Names {
        names_vector: Vec::new(),
    };
    let mut titles = Titles {
        titles_vector: Vec::new(),
    };

    zip_in(&data, &mut names, &mut titles);
}
