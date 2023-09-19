use ass_1::*;


const BOOKS_CSV: &str = "Fundamentals of Wavelets,Goswami, Jaideva,signal_processing,228,Wiley
Data Smart,Foreman, John,data_science,235,Wiley
God Created the Integers,Hawking, Stephen,mathematics,197,Penguin
Superfreakonomics,Dubner, Stephen,economics,179,HarperCollins
Orientalism,Said, Edward,history,197,Penguin
Nature of Statistical Learning Theory, The,Vapnik, Vladimir,data_science,230,Springer
Integration of the Indian States,Menon, V P,history,217,Orient Blackswan
Drunkard's Walk, The,Mlodinow, Leonard,science,197,Penguin
Image Processing & Mathematical Morphology,Shih, Frank,signal_processing,241,CRC
How to Think Like Sherlock Holmes,Konnikova, Maria,psychology,240,Penguin
Data Scientists at Work,Sebastian Gutierrez,data_science,230,Apress
Slaughterhouse Five,Vonnegut, Kurt,fiction,198,Random House
Birth of a Theorem,Villani, Cedric,mathematics,234,Bodley Head
Structure & Interpretation of Computer Programs,Sussman, Gerald,computer_science,240,MIT Press
Age of Wrath, The,Eraly, Abraham,history,238,Penguin
Trial, The,Kafka, Frank,fiction,198,Random House
Statistical Decision Theory',Pratt, John,data_science,236,MIT Press
Data Mining Handbook,Nisbet, Robert,data_science,242,Apress
New Machiavelli, The,Wells, H. G.,fiction,180,Penguin
Physics & Philosophy,Heisenberg, Werner,science,197,Penguin
Making Software,Oram, Andy,computer_science,232,O'Reilly";

const USERS_CSV: &str = "Aube Gilogly,42
Penni Slate,82
Elfreda Fairhall,86
Anette Mingame,38
Xavier Embra,85
Oriana Habben,43
Hughie Mabbitt,47
Jinny Jobe,71
Tomkin Grenshiels,16
Valry Haresnape,75
Clemence Hopkins,15
Norris McConnel,29
Dewain Waind,84
Marcela Gleder,55
Delores Manifould,94
Iolanthe Gunney,75
Philippe Larmett,84
Noel Pummell,44
Pauli Markova,73
Noach Bowton,40
";

fn main() {
   let mut lib = Library::new(BOOKS_CSV);

    let mut users:Vec<Person> = vec![];
    for line in USERS_CSV.lines() {
        let mut split = line.split(",");

        let user = Person::new(
            split.nth(0).unwrap_or("").to_string(),
            split.nth(0).unwrap_or("0").parse::<u32>().unwrap_or(0),
        );

        users.push(user);
    }

    lib.list_books();
    println!("============");
    lib.checkout(1, &users[0]);
    lib.checkout(2, &users[4]);
    
    lib.list_borrowed();
    println!("============");

    lib.return_book(1);

    lib.list_borrowed();
    println!("============");


}