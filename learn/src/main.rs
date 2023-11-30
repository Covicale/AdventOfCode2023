#[derive(Debug)]
enum Country {
    Spain,
    France, 
    Argentina
}

enum Ejemplo{
    Test(Country),
    OtherTest
}

fn test(ej: &Ejemplo) -> String{
    match ej {
        Ejemplo::Test(country) => format!("Pais: {:?}", country),
        Ejemplo::OtherTest => String::from("Other test")
    }
}

fn main (){
    let ej = Ejemplo::Test(Country::Spain);
    let msg = test(&ej);
    println!("{:?}", msg);
}
