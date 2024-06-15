pub struct Film {
    pub name: String,
}

pub fn read_film_name(f: &Film) -> String {
    f.name.clone()
}

pub fn take_film_name(f: Film) -> String {
    f.name
}
