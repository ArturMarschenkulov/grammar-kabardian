/// This module handles the orthography of the language.
/// The goal is to convert every letter into a unit with useful meta-data.
///
/// The main struct is `Letter`, which contains the letter itself, and a `LetterKind`.

// /// [`Letter`] is a struct which contains the letter itself, and a [`LetterKind`].
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Letter {
//     pub kind: LetterKind,
// }
/// [`Letter`] is an enum which contains the kind of the letter.
/// It can be a consonant, a vowel, or a combination of consonant and vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Letter {
    /// Represents a consonant letter.
    Consonant(Consonant),
    /// Represents a vowel letter.
    Vowel(Vowel),
    /// Represents special letters which are a combination of consonant and vowel
    /// Those are 'я', 'е', 'и', 'у', 'о'.
    /// Tese are represented as below:
    /// 'я' = 'йа',
    /// 'е' = 'йэ',
    /// 'и' = 'йы',
    /// 'у' = 'уы',                             // 'ўы'
    /// 'о' = 'уэ', (actually it's rather'эу')  // 'ўэ'
    ///
    ///
    Combi(Consonant, Vowel),
}

impl Letter {
    pub fn voiceness(&self) -> Voiceness {
        match &self {
            Letter::Consonant(c) => c.voiceness,
            Letter::Vowel(..) => Voiceness::Voiced,
            Letter::Combi(..) => Voiceness::Voiced,
        }
    }

    pub fn is_vowel(&self) -> bool {
        match &self {
            Letter::Vowel(..) => true,
            _ => false,
        }
    }
    pub fn is_consonant(&self) -> bool {
        match &self {
            Letter::Consonant(..) => true,
            _ => false,
        }
    }
    pub fn is_combi(&self) -> bool {
        match &self {
            Letter::Combi(..) => true,
            _ => false,
        }
    }
    pub fn is_consonant_or_combi(&self) -> bool {
        self.is_consonant() || self.is_combi()
    }
    pub fn is_vowel_or_combi(&self) -> bool {
        self.is_vowel() || self.is_combi()
    }
}

impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x = match &self {
            Letter::Consonant(c) => c.to_string(),
            Letter::Vowel(v) => v.to_string(),
            Letter::Combi(c, v) => {
                let x = combine_to_combi(&c.to_string(), &v.to_string())
                    .expect("Already checked that it is a combi");
                x.to_string()
            }
        };
        write!(f, "{}", x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vowel {
    /// Represents the 'a' sound
    AA,
    /// Represents the 'э' sound
    A,
    /// Represents the 'ы' sound
    Y,
}
impl TryFrom<&str> for Vowel {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {

        if s.chars().count() != 1 {
            return Err(format!("{} is too long or too short.", s));
        }

        match s {
            "а" => Ok(Vowel::AA),
            "э" => Ok(Vowel::A),
            "ы" => Ok(Vowel::Y),
            _ => Err(format!("{} can not be transfromed into a `Vowel`", s)),
        }
    }
}

impl From<&Vowel> for &str {
    fn from(c: &Vowel) -> Self {
        match c {
            Vowel::AA => "а",
            Vowel::A => "э",
            Vowel::Y => "ы",
        }
    }
}

impl std::fmt::Display for Vowel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Voiceness {
    Voiceless,
    Voiced,
    Ejective,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Consonant {
    pub place: Place,
    pub manner: Manner,
    pub voiceness: Voiceness,
    pub is_labialized: bool,
}
impl Consonant {
    fn try_new(
        place: Place,
        manner: Manner,
        voiceness: Voiceness,
        is_labialized: bool,
    ) -> Result<Self, String> {
        let cons = Consonant {
            place,
            manner,
            voiceness,
            is_labialized,
        };
        if cons.is_valid() {
            Ok(cons)
        } else {
            Err(format!("{} is not a valid consonant", cons))
        }
    }
    fn is_valid(&self) -> bool {
        <&str>::try_from(self).is_ok()
    }

    /// Returns `true` if the consonant is a labial plosive, e.g. 'п' or 'б'.
    pub fn is_labial_plosive(&self) -> bool {
        self.is_place_and_manner(Place::Labial, Manner::Plosive)
    }

    pub fn is_place_and_manner(&self, place: Place, manner: Manner) -> bool {
        self.manner == manner && self.place == place
    }
    pub fn is_manner(&self, manner: Manner) -> bool {
        self.manner == manner
    }
    pub fn is_place(&self, place: Place) -> bool {
        self.place == place
    }

    pub fn is_nasal(&self) -> bool {
        self.is_manner(Manner::Nasal)
    }
    pub fn is_trill(&self) -> bool {
        self.is_manner(Manner::Trill)
    }
    /// Returns `true` if the consonant is a velar, uvular or pharyngeal plosive, aka it needs an 'ы' before 'у'.
    pub fn needs_epenthetic_y(&self) -> bool {
        use Place::*;
        [Velar, Uvular, Glottal].contains(&self.place)
    }
}

impl TryFrom<&str> for Consonant {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use Manner::*;
        use Place::*;
        use Voiceness::*;
        match s {
            // Nasals
            "м" => Consonant::try_new(Labial, Nasal, Voiced, false),
            "н" => Consonant::try_new(Alveolar, Nasal, Voiced, false),
            // Plosives Unvoiced
            "п" => Consonant::try_new(Labial, Plosive, Voiceless, false),
            "т" => Consonant::try_new(Alveolar, Plosive, Voiceless, false),
            "к" => Consonant::try_new(Velar, Plosive, Voiceless, false),
            "къ" => Consonant::try_new(Uvular, Plosive, Voiceless, false),
            "I" => Consonant::try_new(Glottal, Plosive, Voiceless, false),
            "Iу" => Consonant::try_new(Glottal, Plosive, Voiceless, true),
            // Plosives Unvoiced Labialized
            "ку" => Consonant::try_new(Velar, Plosive, Voiceless, true),
            "кьу" => Consonant::try_new(Uvular, Plosive, Voiceless, true),

            // Plosives Voiced
            "б" => Consonant::try_new(Labial, Plosive, Voiced, false),
            "д" => Consonant::try_new(Alveolar, Plosive, Voiced, false),

            // Plosives Voiced Labialized
            "гу" => Consonant::try_new(Velar, Plosive, Voiced, true),
            // Plosives Ejective
            "пI" => Consonant::try_new(Labial, Plosive, Ejective, false),
            "тI" => Consonant::try_new(Alveolar, Plosive, Ejective, false),
            // Plosives Ejective Labialized
            "кIу" => Consonant::try_new(Velar, Plosive, Ejective, true),
            // Affricates Unvoiced
            "ц" => Consonant::try_new(Alveolar, Affricative, Voiceless, false),
            "ч" => Consonant::try_new(PostAlveolar, Affricative, Voiceless, false),
            "кхъ" => Consonant::try_new(Uvular, Affricative, Voiceless, false),
            // Affricates Unvoiced Labialized
            "кхъу" => Consonant::try_new(Uvular, Affricative, Voiceless, true),
            // Affricates Voiced
            "дз" => Consonant::try_new(Alveolar, Affricative, Voiced, false),
            "дж" => Consonant::try_new(PostAlveolar, Affricative, Voiced, false),
            // Affricates Ejecitive
            "цI" => Consonant::try_new(Alveolar, Affricative, Ejective, false),
            "кI" => Consonant::try_new(PostAlveolar, Affricative, Ejective, false),
            // Fricatives Unvoiced
            "ф" => Consonant::try_new(Labial, Fricative, Voiceless, false),
            "с" => Consonant::try_new(Alveolar, Fricative, Voiceless, false),
            "лъ" => Consonant::try_new(Lateral, Fricative, Voiceless, false),
            "ш" => Consonant::try_new(PostAlveolar, Fricative, Voiceless, false),
            "щ" => Consonant::try_new(Palatal, Fricative, Voiceless, false),
            "х" => Consonant::try_new(Velar, Fricative, Voiceless, false),
            "хъ" => Consonant::try_new(Uvular, Fricative, Voiceless, false),
            "хь" => Consonant::try_new(Pharyngeal, Fricative, Voiceless, false),
            // Fricatives Unvoiced Labialized
            "ху" => Consonant::try_new(Velar, Fricative, Voiceless, true),
            "хъу" => Consonant::try_new(Uvular, Fricative, Voiceless, true),
            // Fricatives Voiced
            "в" => Consonant::try_new(Labial, Fricative, Voiced, false),
            "з" => Consonant::try_new(Alveolar, Fricative, Voiced, false),
            "л" => Consonant::try_new(Lateral, Fricative, Voiced, false),
            "ж" => Consonant::try_new(PostAlveolar, Fricative, Voiced, false),
            "жь" => Consonant::try_new(Palatal, Fricative, Voiced, false),
            "г" => Consonant::try_new(Velar, Fricative, Voiced, false), // Plosive?
            "гъ" => Consonant::try_new(Uvular, Fricative, Voiced, false),
            // Fricatives Voiced Labialized
            "гъу" => Consonant::try_new(Uvular, Fricative, Voiced, true),
            // Fricatives Ejective
            "фI" => Consonant::try_new(Labial, Fricative, Ejective, false),
            "лI" => Consonant::try_new(Lateral, Fricative, Ejective, false),
            "щI" => Consonant::try_new(Palatal, Fricative, Ejective, false),
            // Trills
            "р" => Consonant::try_new(Alveolar, Trill, Voiced, false),
            "й" => Consonant::try_new(Palatal, Approximant, Voiced, false),
            // Consider actually using "w" for this, because у can also be a combi.
            "у" => Consonant::try_new(Labial, Approximant, Voiced, false), // labialized ?

            x => Err(format!("Unknown consonant: {}", x)),
        }
    }
}

impl TryFrom<&Consonant> for &str {
    type Error = String;
    fn try_from(c: &Consonant) -> Result<Self, Self::Error> {
        use Manner::*;
        use Place::*;
        use Voiceness::*;
        match (c.place, c.manner, c.voiceness, c.is_labialized) {
            // Plosives
            (Labial, Nasal, Voiced, false) => Ok("н"),
            (Alveolar, Nasal, Voiced, false) => Ok("м"),

            (Labial, Plosive, Voiceless, false) => Ok("п"),
            (Labial, Plosive, Voiced, false) => Ok("б"),
            (Labial, Plosive, Ejective, false) => Ok("пI"),

            (Alveolar, Plosive, Voiceless, false) => Ok("т"),
            (Alveolar, Plosive, Voiced, false) => Ok("д"),
            (Alveolar, Plosive, Ejective, false) => Ok("тI"),

            (Velar, Plosive, Voiceless, false) => Ok("к"),
            (Velar, Plosive, Voiced, true) => Ok("гу"),

            (Uvular, Plosive, Voiceless, false) => Ok("къ"),
            (Glottal, Plosive, Voiceless, false) => Ok("I"),

            (Uvular, Plosive, Voiceless, true) => Ok("къу"),
            (Glottal, Plosive, Voiceless, true) => Ok("Iу"),

            // Fricatives
            (Alveolar, Affricative, Voiceless, false) => Ok("ц"),
            (Alveolar, Affricative, Voiced, false) => Ok("дз"),
            (Alveolar, Affricative, Ejective, false) => Ok("цI"),

            (PostAlveolar, Affricative, Voiceless, false) => Ok("ч"),
            (PostAlveolar, Affricative, Voiced, false) => Ok("дж"),
            (PostAlveolar, Affricative, Ejective, false) => Ok("кI"),
            (Uvular, Affricative, Voiceless, false) => Ok("кхъ"),

            (Uvular, Affricative, Voiceless, true) => Ok("кхъу"),



            (Labial, Fricative, Voiceless, false) => Ok("ф"),
            (Labial, Fricative, Voiced, false) => Ok("в"),
            (Labial, Fricative, Ejective, false) => Ok("фI"),

            (Alveolar, Fricative, Voiceless, false) => Ok("с"),
            (Alveolar, Fricative, Voiced, false) => Ok("з"),

            (Lateral, Fricative, Voiceless, false) => Ok("лъ"),
            (Lateral, Fricative, Voiced, false) => Ok("л"),
            (Lateral, Fricative, Ejective, false) => Ok("лI"),

            (PostAlveolar, Fricative, Voiceless, false) => Ok("ш"),
            (PostAlveolar, Fricative, Voiced, false) => Ok("ж"),

            (Palatal, Fricative, Voiceless, false) => Ok("щ"),
            (Palatal, Fricative, Voiced, false) => Ok("жь"),
            (Palatal, Fricative, Ejective, false) => Ok("щI"),

            (Velar, Fricative, Voiceless, false) => Ok("х"),
            (Velar, Fricative, Voiced, false) => Ok("г"),
            (Uvular, Fricative, Voiceless, false) => Ok("хъ"),
            (Uvular, Fricative, Voiced, false) => Ok("гъ"),
            (Pharyngeal, Fricative, Voiceless, false) => Ok("хь"),

            (Velar, Fricative, Voiceless, true) => Ok("ху"),
            (Uvular, Fricative, Voiceless, true) => Ok("хъу"),
            (Uvular, Fricative, Voiced, true) => Ok("гьу"),

            (Palatal, Approximant, Voiced, false) => Ok("й"),
            (Labial, Approximant, Voiced, false) => Ok("у"),
            (Alveolar, Trill, Voiced, false) => Ok("р"),


            (Velar, Plosive, Voiced, false) => Err("kabardian doesn't have a voiced velar plosive, did you mean voiced velar fricative?".to_owned()),
            (Labial, _, _, true) => Err("".to_owned()),
            (Alveolar, _, _, true) => Err("".to_owned()),
            (PostAlveolar, _, _, true) => Err("".to_owned()),
            (Pharyngeal, _, _, true) => Err("".to_owned()),
            // (Ignore, _, _, _) => panic!(""),
            // (_, Manner::Ignore, _, _) => panic!(""),

            x => Err(format!("{:?}", x)),
        }
    }
}

impl std::fmt::Display for Consonant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x = match <&str>::try_from(self) {
            Ok(s) => s,
            Err(e) => unreachable!("{}", e),
        };
        write!(f, "{}", x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    Labial,
    Alveolar,
    Lateral,
    PostAlveolar,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Glottal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Manner {
    Nasal,
    Plosive,
    Affricative,
    Fricative,
    Approximant,
    Trill,
}

fn first_of_combi(c: &char) -> Result<char, String> {
    match c {
        'я' | 'е' | 'и' => Ok('й'),
        'о' | 'у' => Ok('у'),
        _ => Err(format!("{} is not a combi", c)),
    }
}
fn last_of_combi(c: &char) -> Result<char, String> {
    match c {
        'я' => Ok('а'),
        'е' | 'о' => Ok('э'),
        'и' | 'у' => Ok('ы'),
        _ => Err(format!("{} is not a combi", c)),
    }
}
fn split_combi(combi: &char) -> Result<(Consonant, Vowel), String> {
    if !is_char_combi(combi) {
        return Err(format!("{} is not a combi", combi));
    }
    let first = first_of_combi(combi).expect("Parameter is a combi.");
    let last = last_of_combi(combi).expect("Parameter is a combi.");

    let c = Consonant::try_from(first.to_string().as_str()).expect("This must be a consonant.");
    let v = Vowel::try_from(last.to_string().as_str()).expect("This must be a vowel.");
    Ok((c, v))
}
fn combine_to_combi(c_0: &String, c_1: &String) -> Option<char> {
    match (&c_0.as_ref(), &c_1.as_ref()) {
        (&"й", &"э") => Some('e'),
        (&"й", &"ы") => Some('и'),
        (&"й", &"а") => Some('я'),
        (&"у", &"э") => Some('о'),
        (&"у", &"ы") => Some('у'),
        _ => None,
    }
}
fn is_char_combi(c: &char) -> bool {
    match c {
        'я' | 'е' | 'и' | 'о' | 'у' => true,
        _ => false,
    }
}
fn is_char_vowel(c: &char) -> bool {
    match c {
        'а' | 'э' | 'ы' => true,
        _ => false,
    }
}
fn is_char_diacritic(c: &char) -> bool {
    match c {
        'ь' | 'ъ' => true,
        'I' | 'у' => true,
        _ => false,
    }
}
fn is_char_consonant(c: &char) -> bool {
    match c {
        'м' | 'н' => true,
        'I' => true,
        'п' | 'б' | 'т' | 'д' | 'к' | 'г' => true,
        'ф' | 'в' | 'с' | 'з' | 'ш' | 'щ' | 'х' | 'ж' | 'р' | 'л' | 'й' | 'ч' | 'ц' => {
            true
        }
        _ => false,
    }
}

fn parse_vowel(vowel: &char) -> Result<Letter, String> {
    let v = Vowel::try_from(vowel.to_string().as_str())?;
    Ok(Letter::Vowel(v))
}

fn parse_combi(combi: &char) -> Result<Letter, String> {
    let (c, v) = split_combi(combi)?;
    Ok(Letter::Combi(c, v))
}

fn parse_consonant(consonant: &char, chars: &[char], i: &mut usize) -> Result<Letter, String> {
    #[derive(PartialEq)]
    enum Deco {
        Base,
        Palotshka,
        MagkiyZnak,
        TverdyyZnak,
    }
    let can_palochka = ['п', 'т', 'ф', 'щ', 'к', 'х', 'ц', 'л'];
    let can_labialized = ['к', 'г', 'х', 'I'];
    let can_tverdyj_znak = ['к', 'х', 'г', 'л'];
    let can_magkiy_znak = ['х', 'ж'];
    let simple_cons = ['м', 'н', 'б', 'ч', 'с', 'з', 'ш', 'в', 'р'];

    let mut consonant_str: Vec<char> = Vec::new();
    consonant_str.push(*consonant);

    // attribs of the current letter
    let base_char = consonant_str[0];
    let mut has_labial = false;
    let mut deco = Deco::Base;

    'bp: loop {
        if simple_cons.contains(&base_char) {
            break 'bp;
        }
        let next_letter = chars.get(*i + 1);
        if next_letter.map(is_char_diacritic).unwrap_or(false) {}
        if next_letter.map(is_char_vowel).unwrap_or(false) {
            break 'bp;
        }

        match &next_letter {
            Some(c)
                if can_palochka.contains(&base_char)
                    && deco == Deco::Base
                    && !has_labial
                    && *c == &'I' =>
            {
                consonant_str.push(**c);
                *i += 1;
                deco = Deco::Palotshka;
            }
            Some(c)
                if can_labialized.contains(&base_char)
                    && !has_labial
                    && deco != Deco::MagkiyZnak
                    && *c == &'у' =>
            {
                consonant_str.push(**c);
                *i += 1;
                has_labial = true;
            }
            Some(c)
                if can_tverdyj_znak.contains(&base_char)
                    && deco == Deco::Base
                    && !has_labial
                    && *c == &'ъ' =>
            {
                consonant_str.push(**c);
                *i += 1;
                deco = Deco::TverdyyZnak;
            }
            Some(c)
                if can_magkiy_znak.contains(&base_char)
                    && deco == Deco::Base
                    && !has_labial
                    && *c == &'ь' =>
            {
                consonant_str.push(**c);
                *i += 1;
                deco = Deco::MagkiyZnak;
            }
            Some(c @ 'з') | Some(c @ 'ж') if base_char == 'д' => {
                consonant_str.push(**c);
                *i += 1;
            }
            Some(c @ 'х')
                if base_char == 'к' && chars.get(*i + 2).map(|x| x == &'ъ').unwrap_or(false) =>
            {
                consonant_str.push(**c);
                *i += 1;
            }
            _ => break 'bp,
        }
    }
    let consonant = Consonant::try_from(consonant_str.iter().collect::<String>().as_str())
        .expect("If this panics, there is a bug in the code");
    Ok(Letter::Consonant(consonant))
}

fn parse_letter(ch: &char, chars: &[char], i: &mut usize) -> Result<Letter, String> {
    match ch {
        v if is_char_vowel(v) => Ok(parse_vowel(v).expect("Parameter is vowel")),
        cv if is_char_combi(cv) => Ok(parse_combi(cv).expect("Parameter is a combi.")),
        c if is_char_consonant(c) => {
            Ok(parse_consonant(c, chars, i).expect("Parameter is a consonant"))
        }
        x => Err(format!("{} is not a valid letter.", x)),
    }
}

/// Parses a string into a vector of letters.
///
/// This is especially useful since many letters in Kabardian are di-, tri-, or even quadrigraphs.
pub fn parse(s: &str) -> Result<Vec<Letter>, String> {
    let mut letters = Vec::new();
    let chars = s.chars().collect::<Vec<char>>();
    let possible_charaters = [
        'ь', 'ъ', //
        'а', 'э', 'ы', //
        'я', 'е', 'и', 'о', 'у', 'ю', //
        'м', 'н', //
        'п', 'б', 'т', 'д', 'к', 'г', 'I', //
        'ф', 'в', 'с', 'з', 'ш', 'щ', 'х', 'ж', //
        'ч', 'ц', //
        'р', 'л', 'й', //
    ];
    for c in &chars {
        if !possible_charaters.contains(c) {
            return Err(format!("invalid character: {}", c));
        }
    }

    let mut i = 0;
    while i < chars.len() {
        match &chars.get(i) {
            Some(c) => {
                let letter = parse_letter(c, &chars, &mut i)
                    .expect("Already checked that the input is valid.");
                letters.push(letter);
                i += 1;
            }
            None => break,
        }
    }

    Ok(letters)
}
