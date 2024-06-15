#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

pub fn choose_outfit(form_level: Option<u32>, inv_msg: Result<&str, &str>) -> Outfit {
    let jacket = match form_level {
        None => Jacket::Flowers,
        Some(level) if level > 0 => Jacket::White,
        _ => Jacket::Black,
    };
    let hat = match (form_level, inv_msg) {
        (None, Err(_)) => Hat::Baseball,
        (_, Ok(_)) => Hat::Fedora,
        _ => Hat::Snapback,
    };
    Outfit { jacket, hat }
}
