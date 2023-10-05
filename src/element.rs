#[derive(Debug)]
pub enum CreationError {
    AtomicNumberNotFound(u8),
    NameNotFound(String),
    SymbolNotFound(String),
}

pub struct ChemicalElement {
    atomic_number: u8,
    name: String,
    symbol: String,
}

pub fn sanitize(text: &str) -> String {
    text.to_ascii_lowercase()
        .replace('á', "a")
        .replace('é', "e")
        .replace('í', "i")
        .replace('ó', "o")
        .replace('ú', "u")
}

impl ChemicalElement {
    pub fn new(atomic_number: u8, name: String, symbol: String) -> Self {
        Self {
            atomic_number,
            name,
            symbol,
        }
    }

    pub fn atomic_number(&self) -> u8 {
        self.atomic_number
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn from_atomic_number(atomic_number: u8) -> Result<Self, CreationError> {
        let Some((name, symbol)) = Self::all_attributes().get(atomic_number as usize - 1) else {
            return Err(CreationError::AtomicNumberNotFound(atomic_number));
        };

        Ok(Self::new(
            atomic_number,
            name.to_string(),
            symbol.to_string(),
        ))
    }

    pub fn from_name(name: &str) -> Result<Self, CreationError> {
        if let Some((i, (name, symbol))) = Self::all_attributes()
            .iter()
            .enumerate()
            .find(|(_, (el_name, _))| sanitize(el_name) == sanitize(name))
        {
            return Ok(Self::new(i as u8 + 1, name.to_string(), symbol.to_string()));
        } else {
            Err(CreationError::NameNotFound(name.to_string()))
        }
    }

    pub fn from_symbol(symbol: &str) -> Result<Self, CreationError> {
        if let Some((i, (name, symbol))) = Self::all_attributes()
            .iter()
            .enumerate()
            .find(|(_, (_, el_symbol))| sanitize(el_symbol) == sanitize(symbol))
        {
            return Ok(Self::new(i as u8 + 1, name.to_string(), symbol.to_string()));
        } else {
            Err(CreationError::SymbolNotFound(symbol.to_string()))
        }
    }

    pub fn all_attributes() -> &'static [(&'static str, &'static str)] {
        // The names are in Spanish because this is for my school.
        &[
            ("Hidrógeno", "H"),
            ("Helio", "He"),
            ("Litio", "Li"),
            ("Berilio", "Be"),
            ("Boro", "B"),
            ("Carbono", "C"),
            ("Nitrógeno", "N"),
            ("Oxígeno", "O"),
            ("Flúor", "F"),
            ("Neón", "Ne"),
            ("Sodio", "Na"),
            ("Magnesio", "Mg"),
            ("Aluminio", "Al"),
            ("Silicio", "Si"),
            ("Fósforo", "P"),
            ("Azufre", "S"),
            ("Cloro", "Cl"),
            ("Argón", "Ar"),
            ("Potasio", "K"),
            ("Calcio", "Ca"),
            ("Escandio", "Sc"),
            ("Titanio", "Ti"),
            ("Vanadio", "V"),
            ("Cromo", "Cr"),
            ("Manganeso", "Mn"),
            ("Hierro", "Fe"),
            ("Cobalto", "Co"),
            ("Niquel", "Ni"),
            ("Cobre", "Cu"),
            ("Zinc", "Zn"),
            ("Galio", "Ga"),
            ("Germanio", "Ge"),
            ("Arsénico", "As"),
            ("Selenio", "Se"),
            ("Bromo", "Br"),
            ("Criptón", "Kr"),
            ("Rubidio", "Rb"),
            ("Estroncio", "Sr"),
            ("Itrio", "Y"),
            ("Zirconio", "Zr"),
            ("Niobio", "Nb"),
            ("Molibdeno", "Mo"),
            ("Tecnecio", "Tc"),
            ("Rutenio", "Ru"),
            ("Rodio", "Rh"),
            ("Paladio", "Pd"),
            ("Plata", "Ag"),
            ("Cadmio", "Cd"),
            ("Indio", "In"),
            ("Estaño", "Sn"),
            ("Antimonio", "Sb"),
            ("Telurio", "Te"),
            ("Yodo", "I"),
            ("Xenón", "Xe"),
            ("Cesio", "Cs"),
            ("Bario", "Ba"),
            ("Lantano", "La"),
            ("Cerio", "Ce"),
            ("Praseodimio", "Pr"),
            ("Neodimio", "Nd"),
            ("Prometio", "Pm"),
            ("Samario", "Sm"),
            ("Europio", "Eu"),
            ("Gadolino", "Gd"),
            ("Terbio", "Tb"),
            ("Disprosio", "Dy"),
            ("Holmio", "Ho"),
            ("Erbio", "Er"),
            ("Tulio", "Tm"),
            ("Iterbio", "Yb"),
            ("Lutecio", "Lu"),
            ("Hafnio", "Hf"),
            ("Tantalio", "Ta"),
            ("Tungsteno", "W"),
            ("Renio", "Re"),
            ("Osmio", "Os"),
            ("Iridio", "Ir"),
            ("Platino", "Pt"),
            ("Oro", "Au"),
            ("Mercurio", "Hg"),
            ("Talio", "Tl"),
            ("Plomo", "Pb"),
            ("Bismuto", "Bi"),
            ("Polonio", "Po"),
            ("Astato", "At"),
            ("Radón", "Rn"),
            ("Francio", "Fr"),
            ("Radio", "Ra"),
            ("Actinio", "Ac"),
            ("Torio", "Th"),
            ("Protactinio", "Pa"),
            ("Uranio", "U"),
            ("Neptunio", "Np"),
            ("Plutonio", "Pu"),
            ("Americio", "Am"),
            ("Curio", "Cm"),
            ("Berquelio", "Bk"),
            ("Californio", "Cf"),
            ("Einstenio", "Es"),
            ("Fermio", "Fm"),
            ("Mendelevio", "Md"),
            ("Nobelio", "No"),
            ("Laurencio", "Lr"),
            ("Rutherfordio", "Rf"),
            ("Dubnio", "Db"),
            ("Seaborgio", "Sg"),
            ("Bohrio", "Bh"),
            ("Hassio", "Hs"),
            ("Meitnerio", "Mt"),
            ("Darmstadtio", "Ds"),
            ("Roentgenio", "Rg"),
            ("Copernicio", "Cn"),
            ("Nihonio", "Nh"),
            ("Flerovio", "Fl"),
            ("Moscovio", "Mc"),
            ("Livermorio", "Lv"),
            ("Teneso", "Ts"),
            ("Oganesón", "Og"),
        ]
    }
}
