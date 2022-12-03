#![allow(dead_code, unused_variables)]
#![allow(clippy::match_like_matches_macro)]

mod evaluation;
mod ortho;
mod table;
mod template;

use std::collections::VecDeque;

use table::Wikitable;

#[derive(Debug, Clone, PartialEq)]
enum PreverbSoundForm {
    Full,        // e.g. къэ-
    Reduced,     // e.g. къы-
    BeforeVowel, // e.g. къ-
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Preverb {
    base: String,
}
impl Preverb {
    fn new(base: &String) -> Self {
        Preverb {
            // form: PreverbSoundForm::Full,
            base: base.to_owned(),
        }
    }
    fn get_first_letter(&self) -> ortho::Letter {
        ortho::parse(&self.base)[0].clone()
    }
    fn get_last_consonant(&self) -> Option<ortho::Consonant> {
        let letters = ortho::parse(&self.base);
        let mut last_consonant = None;
        for letter in letters {
            match letter.kind {
                ortho::LetterKind::Consonant(consonant) => last_consonant = Some(consonant),
                _ => {}
            }
        }
        last_consonant
    }
    fn get_form(&self, form: &PreverbSoundForm) -> String {
        match &form {
            PreverbSoundForm::Full => self.base.clone(),
            PreverbSoundForm::Reduced => self.get_reduced(),
            PreverbSoundForm::BeforeVowel => self.get_before_vowel(),
        }
    }
    fn get_before_vowel(&self) -> String {
        let sss = match &self.base {
            base if base.ends_with('э') || base.ends_with('ы') => {
                let mut chars = base.chars();
                chars.next_back();
                let reduced = chars.as_str().to_string();
                reduced
            }
            _ => unreachable!(),
        };
        sss
    }
    fn get_reduced(&self) -> String {
        let sss = match &self.base {
            base if base.ends_with('э') || base.ends_with('ы') => {
                let mut chars = base.chars();
                chars.next_back();
                let reduced = chars.as_str().to_string();
                reduced + "ы"
            }
            _ => unreachable!(),
        };
        sss
    }

    fn get_string(&self, form: PreverbSoundForm) -> String {
        let sss = match &self.base {
            // This handles the preverbs which end on 'э'
            base if base.ends_with('э') => {
                let mut chars = base.chars();
                chars.next_back();
                let reduced = chars.as_str().to_string();

                match form {
                    PreverbSoundForm::Full => base.to_owned(),
                    PreverbSoundForm::Reduced => reduced + "ы",
                    PreverbSoundForm::BeforeVowel => reduced,
                }
            }
            base if base.ends_with('ы') => {
                let mut chars = base.chars();
                chars.next_back();
                let reduced = chars.as_str().to_string();

                match form {
                    PreverbSoundForm::Full => base.to_owned(),
                    PreverbSoundForm::Reduced => reduced + "ы",
                    PreverbSoundForm::BeforeVowel => reduced,
                }
            }
            _ => unreachable!(),
        };
        sss
    }
}

#[derive(Debug, Clone, PartialEq)]
enum MorphemeKind {
    Preverb(Preverb),
    PersonMarker(PersonMarker),
    Stem(template::VerbStem),
    // InfitiveSuffix,
    NegationPrefix,
    RajImperative,
    Generic,
}
#[derive(Debug, Clone)]
pub struct Morpheme {
    kind: MorphemeKind,
    base: String,
}
impl Morpheme {
    fn get_first_letter(&self) -> Option<ortho::Letter> {
        let letters = ortho::parse(&self.base);
        let first_letter = letters.first().cloned();
        first_letter
    }
    fn to_letters(&self) -> Vec<ortho::Letter> {
        ortho::parse(&self.base)
    }
}
impl Morpheme {
    fn make_generic(base: &str) -> Self {
        Morpheme {
            kind: MorphemeKind::Generic,
            base: base.to_owned(),
        }
    }
    fn make_negative_prefix() -> Self {
        Morpheme {
            kind: MorphemeKind::NegationPrefix,
            base: "мы".to_owned(),
        }
    }
    fn make_imperative_raj() -> Self {
        Morpheme {
            kind: MorphemeKind::RajImperative,
            base: "ре".to_owned(),
        }
    }
    fn make_preverb(preverb: &Preverb) -> Self {
        Morpheme {
            kind: MorphemeKind::Preverb(preverb.clone()),
            base: preverb.base.clone(),
        }
    }
    fn make_person_marker(marker: &PersonMarker) -> Self {
        Morpheme {
            kind: MorphemeKind::PersonMarker(*marker),
            base: marker.get_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Case {
    /// (-р) subject of intransitive verb, direct object of transitive verb
    Absolutive,
    /// (-м) subject of transitive verb
    Ergative,
    /// (-м) indirect object of intransitive and transitive verbs.
    Oblique,
}

/// A struct that indicates the number of a noun or verb.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Number {
    Singular,
    Plural,
}
/// A struct that indicates the person of a verb.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PersonMarker {
    person: Person,
    number: Number,
    case: Case,
}

impl PersonMarker {
    fn new(person: Person, number: Number, case: Case) -> Self {
        PersonMarker {
            person,
            number,
            case,
        }
    }
}
impl PersonMarker {
    /// Returns the "base" form of the person markers
    fn get_base_2(&self) -> String {
        Self::get_base_from(&self.person, &self.number, &self.case)
    }
    fn get_base_from(person: &Person, number: &Number, case: &Case) -> String {
        let pm = PersonMarker {
            person: *person,
            number: *number,
            case: *case,
        };
        pm.get_base_string()
    }
    fn get_base_string(&self) -> String {
        use Case::*;
        use Number::*;
        use Person::*;
        let result = match (self.person, self.number, self.case) {
            (First, Singular, Absolutive) => "сы",
            (First, Singular, Ergative) => "с",
            (First, Singular, Oblique) => "сэ",
            (Second, Singular, Absolutive) => "у",
            (Second, Singular, Ergative) => "б",
            (Second, Singular, Oblique) => "уэ",
            (Third, Singular, Absolutive) => "",
            (Third, Singular, Ergative) => "и",
            (Third, Singular, Oblique) => "е",
            (First, Plural, Absolutive) => "ды",
            (First, Plural, Ergative) => "д",
            (First, Plural, Oblique) => "дэ",
            (Second, Plural, Absolutive) => "фы",
            (Second, Plural, Ergative) => "ф",
            (Second, Plural, Oblique) => "фэ",
            (Third, Plural, Absolutive) => "",
            (Third, Plural, Ergative) => "я",
            (Third, Plural, Oblique) => "е",
        };

        result.to_string()
    }
    fn get_string(&self) -> String {
        self.get_base_2()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum VowelKind {
    With,
    Without,
    Alternating,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transitivity {
    Transitive,
    Intransitive,
}
impl Transitivity {
    fn get_subject_case(&self) -> Case {
        match self {
            Transitivity::Transitive => Case::Ergative,
            Transitivity::Intransitive => Case::Absolutive,
        }
    }
}

/*
    {| class="wikitable"
    |-
    ! Инфинитив (масдар) !!
    |-
    | щыӀэныгъэ: || {{{псалъэпкъ}}}эн
    |-
    | щымыӀэныгъэ: || мы{{{псалъэпкъ}}}эн
    |}
*/
fn get_masdar(desc: &template::TemplateDesc) -> String {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let table_name = "Инфинитив (масдар)".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name);
    table.add("".to_owned());

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ:", polarity));

        let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
        morphemes.push_back(Morpheme {
            kind: MorphemeKind::Stem(desc.stem.clone()),
            base: root.clone(),
        });
        morphemes.push_back(Morpheme::make_generic("н"));
        if polarity == "мы" {
            let m = Morpheme::make_negative_prefix();
            morphemes.push_front(m);
        }

        if let Some(preverb) = desc.preverb.clone() {
            let m = Morpheme::make_preverb(&preverb);
            morphemes.push_front(m);
        }
        table.add(evaluation::evaluate_morphemes(&morphemes));
    }

    table.to_string()
}

/*
    {| class="wikitable"
    |-
    ! Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа !! сэ  !! уэ !! ар !! дэ !! фэ !! ахэр
    |-
    | щыӀэныгъэ: || сы{{{псалъэпкъ}}}эн || у{{{псалъэпкъ}}}эн || {{{псалъэпкъ}}}эн || ды{{{псалъэпкъ}}}эн || фы{{{псалъэпкъ}}}эн || {{{псалъэпкъ}}}эн(хэ)
    |-
    | щымыӀэныгъэ: || сымы{{{псалъэпкъ}}}эн || умы{{{псалъэпкъ}}}эн || мы{{{псалъэпкъ}}}эн || дымы{{{псалъэпкъ}}}эн || фымы{{{псалъэпкъ}}}эн || мы{{{псалъэпкъ}}}эн(хэ)
    |}
*/
fn get_masdar_personal(desc: &template::TemplateDesc) -> String {
    let root = "{{{псалъэпкъ}}}".to_owned();

    let table_name = "Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name);
    let subject_case = &desc.transitivity.get_subject_case();

    let pronouns = match &desc.transitivity {
        Transitivity::Intransitive => ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"],
        Transitivity::Transitive => ["сэ", "уэ", "абы", "дэ", "фэ", "абыхэм"],
    };

    for pronoun in pronouns.iter() {
        table.add(pronoun.to_string());
    }
    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));

        for number in &[Number::Singular, Number::Plural] {
            for person in &[Person::First, Person::Second, Person::Third] {
                let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
                morphemes.push_back(Morpheme {
                    kind: MorphemeKind::Stem(desc.stem.clone()),
                    base: root.clone(),
                });
                morphemes.push_back(Morpheme::make_generic("н"));

                // Add negative prefix
                if polarity == "мы" {
                    let m = Morpheme::make_negative_prefix();
                    morphemes.push_front(m);
                }

                // Add ergative person marker
                if subject_case == &Case::Ergative {
                    let marker = PersonMarker::new(*person, *number, *subject_case);
                    let m = Morpheme::make_person_marker(&marker);
                    morphemes.push_front(m);
                };

                // Add preverb
                if let Some(preverb) = desc.preverb.clone() {
                    let m = Morpheme::make_preverb(&preverb);
                    morphemes.push_front(m);
                }

                // Add absolutive person marker2
                if subject_case == &Case::Absolutive {
                    let marker = PersonMarker::new(*person, *number, Case::Absolutive);
                    let m = Morpheme::make_person_marker(&marker);
                    morphemes.push_front(m);
                }

                let s = evaluation::evaluate_morphemes(&morphemes);

                println!("{:?}", evaluation::morphemes_to_string(&morphemes));
                table.add(s);
            }
        }
    }
    table.to_string()
}

/*
{| class="wikitable"
|-
! унафэ наклоненэ !! уэ !! фэ
|-
| щыӀэныгъэ: || {{{псалъэпкъ}}}э! || фы{{{псалъэпкъ}}}э!
|-
| щымыӀэныгъэ: || умы{{{псалъэпкъ}}}э! || фымы{{{псалъэпкъ}}}э!
|}
*/
fn get_imperative(desc: &template::TemplateDesc) -> String {
    let root = "{{{псалъэпкъ}}}".to_owned();
    let table_name = "унафэ наклоненэ".to_owned();

    let mut table = Wikitable::new();
    table.add(table_name);
    for pronoun in ["уэ", "фэ"].iter() {
        table.add(pronoun.to_string());
    }

    let subject_case = &desc.transitivity.get_subject_case();

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));
        for number in &[Number::Singular, Number::Plural] {
            let mut morphemes: VecDeque<Morpheme> = VecDeque::new();
            morphemes.push_back(Morpheme {
                kind: MorphemeKind::Stem(desc.stem.clone()),
                base: root.clone(),
            });

            // Add negative prefix
            if polarity == "мы" {
                let m = Morpheme::make_negative_prefix();
                morphemes.push_front(m);
            }

            // Add ergative person marker
            if subject_case == &Case::Ergative {
                if !((number, polarity) == (&Number::Singular, "")) {
                    let marker = PersonMarker::new(Person::Second, *number, *subject_case);
                    let m = Morpheme::make_person_marker(&marker);
                    morphemes.push_front(m);
                }
            };

            // Add preverb
            if let Some(preverb) = desc.preverb.clone() {
                let m = Morpheme::make_preverb(&preverb);
                morphemes.push_front(m);
            }

            // Add absolutive person marker
            if subject_case == &Case::Absolutive {
                if (*number, polarity) != (Number::Singular, "") {
                    let marker = PersonMarker::new(Person::Second, *number, Case::Absolutive);
                    let m = Morpheme::make_person_marker(&marker);
                    morphemes.push_front(m);
                }
            }

            let s = evaluation::evaluate_morphemes(&morphemes);
            table.add(s);
        }
    }
    table.to_string()
}

/*
{| class="wikitable"
|-
! Ре-кӀэ унафэ наклоненэ !! сэ  !! уэ !! ар !! дэ !! фэ !! ахэр
|-
| щыӀэныгъэ: || сре{{{псалъэпкъ}}}э || уре{{{псалъэпкъ}}}э || ире{{{псалъэпкъ}}}э || дре{{{псалъэпкъ}}}э || фре{{{псалъэпкъ}}}э || ире{{{псалъэпкъ}}}э
|-
| щымыӀэныгъэ: || сремы{{{псалъэпкъ}}}э || уремы{{{псалъэпкъ}}}э || иремы{{{псалъэпкъ}}}э || дремы{{{псалъэпкъ}}}э || фремы{{{псалъэпкъ}}}э || иремы{{{псалъэпкъ}}}э
|}
*/
fn get_imperative_raj(desc: &template::TemplateDesc) -> String {
    let root = "{{{псалъэпкъ}}}".to_owned();

    let mut table = Wikitable::new();
    table.add("Ре-кӀэ унафэ наклоненэ".to_owned());
    for pronoun in ["сэ", "уэ", "ар", "дэ", "фэ", "ахэр"].iter() {
        table.add(pronoun.to_string());
    }

    for polarity in ["", "мы"] {
        table.add_row();
        table.add(format!("щы{}Ӏэныгъэ", polarity));
        for number in &[Number::Singular, Number::Plural] {
            for person in &[Person::First, Person::Second, Person::Third] {
                let mut morphemes: VecDeque<Morpheme> = VecDeque::new();

                // Add stem
                morphemes.push_back(Morpheme {
                    kind: MorphemeKind::Stem(desc.stem.clone()),
                    base: root.clone(),
                });

                // Add negative prefix
                if polarity == "мы" {
                    let m = Morpheme::make_negative_prefix();
                    morphemes.push_front(m);
                }
                // Add imperative raj
                morphemes.push_front(Morpheme::make_imperative_raj());
                // Add preverb
                if let Some(preverb) = desc.preverb.clone() {
                    let m = Morpheme::make_preverb(&preverb);
                    morphemes.push_front(m);
                }

                // Add
                if !(desc.preverb.is_some() && Person::Third == *person) {
                    let marker = PersonMarker::new(
                        *person,
                        // If there is a preverb present, the third person marker is not present
                        if (person, number) == (&Person::Third, &Number::Plural) {
                            Number::Singular
                        } else {
                            *number
                        },
                        Case::Ergative,
                    );
                    let m = Morpheme::make_person_marker(&marker);
                    morphemes.push_front(m);
                }

                let s = evaluation::evaluate_morphemes(&morphemes);
                table.add(s);
            }
        }
    }
    table.to_string()
}

fn create_template(desc: template::TemplateDesc) -> String {
    let mut result = "".to_string();
    result += &format!("<!-- Template:Wt/kbd/{} -->\n", desc.original_string);

    // Инфинитив (масдар)
    result += &get_masdar(&desc);
    result += "\n-\n";

    // Инфинитив (масдар) щхьэкӀэ зэхъуэкӀа
    result += &get_masdar_personal(&desc);
    result += "\n-\n";
    // унафэ наклоненэ
    result += &get_imperative(&desc);
    result += "\n-\n";

    // Ре-кӀэ унафэ наклоненэ
    result += &get_imperative_raj(&desc);
    result += "\n-\n";

    result += "|}<noinclude>\n[[Category:Wt/kbd]]\n</noinclude>";
    println!("{}", result);

    result
}

/*
    Ideas about the strucutre.
    - Having several modes. I want this projec to be quite flexible. The idea is not only to support wikipsalhalhe, but also other projects in the future if need be.
        Right now wikipsalhalhe is the main focus, but it should be extensible to other projects.

    1. Template extraction:
        We give the engine a template string. It extract the necessary information from it.
    2.
*/
fn main() {
    // спр-лъэӏ-зэхэ-д0д-ы
    let template = "спр-лъэӏ-зэхэ-дбд-ы"; // tr. base. vl. e.g. хьын
    let template = "спр-лъэмыӏ-0-0д-ы"; // intr. base. vl. e.g. плъэн
    let template = template::create_template_from_string(template.to_owned()).unwrap();
    create_template(template);
}
