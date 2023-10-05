use std::fmt::Display;

use crate::element::ChemicalElement;

pub struct Subshells {
    items: Vec<Subshell>,
}

impl Display for Subshells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.items
                .iter()
                .map(|i| format!("{i}"))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

pub struct Subshell {
    kind: char,
    period: u8,
    value: u8,
}

impl Subshell {
    pub fn new(kind: char, period: u8, value: u8) -> Self {
        Self {
            kind,
            period,
            value,
        }
    }

    pub fn get_value(kind: char) -> u8 {
        match kind {
            'S' => 2,
            'P' => 6,
            'd' => 10,
            'f' => 14,
            _ => unreachable!(),
        }
    }
}

impl Display for Subshell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self.value {
            0 => "⁰",
            1 => "¹",
            2 => "²",
            3 => "³",
            4 => "⁴",
            5 => "⁵",
            6 => "⁶",
            7 => "⁷",
            8 => "⁸",
            9 => "⁹",
            10 => "¹⁰",
            11 => "¹¹",
            12 => "¹²",
            13 => "¹³",
            14 => "¹⁴",
            _ => unreachable!(),
        };

        write!(f, "{}{}{value}", self.period, self.kind)
    }
}

pub trait AsSubshells {
    fn as_subshells(&self) -> Subshells;
}

impl AsSubshells for ChemicalElement {
    fn as_subshells(&self) -> Subshells {
        let subshells = [
            (1, 'S'),
            (2, 'S'),
            (2, 'P'),
            (3, 'S'),
            (3, 'P'),
            (4, 'S'),
            (3, 'd'),
            (4, 'P'),
            (5, 'S'),
            (4, 'd'),
            (5, 'P'),
            (6, 'S'),
            (4, 'f'),
            (5, 'd'),
            (6, 'P'),
            (7, 'S'),
            (5, 'f'),
            (6, 'd'),
            (7, 'P'),
        ];

        let mut remaining = self.atomic_number();
        let mut items = Vec::new();

        for (period, kind) in subshells.into_iter() {
            let value = Subshell::get_value(kind);

            if value >= remaining {
                items.push(Subshell::new(kind, period, remaining));
                break;
            } else {
                items.push(Subshell::new(kind, period, value));
                remaining -= value;
            }
        }

        Subshells { items }
    }
}
