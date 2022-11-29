use crate::ortho;
use crate::template;
use crate::PersonMarker;
use crate::{Case, Morpheme, MorphemeKind, Number, Person, Preverb, PreverbForm};
use std::collections::VecDeque;

pub fn morphemes_to_string(morphemes: &VecDeque<Morpheme>) -> String {
    let s = morphemes
        .iter()
        .map(|m| m.base.clone())
        .collect::<Vec<String>>()
        .join("-");
    s
}
fn evaluate_person_marker(
    marker: &PersonMarker,
    morphemes: &VecDeque<Morpheme>,
    i: usize,
) -> String {
    let morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
    let morpheme_next = morphemes.get(i + 1);
    let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
    let morpheme_next_kind = morpheme_next.map(|x| &x.kind);
    let is_raj_imperative = morphemes.iter().any(|m| {
        if let MorphemeKind::RajImperative = &m.kind {
            return true;
        }
        false
    });

    let is_third_plural = (marker.person, marker.number) == (Person::Third, Number::Plural);
    let is_second_singular = (marker.person, marker.number) == (Person::Second, Number::Singular);

    let m = marker;
    let ss = match marker.case {
        Case::Ergative => {
            // Because ergative markers are only one letter, we do this
            let letter = &ortho::parse(&marker.get_base_string())[0];

            if is_third_plural {
                let x = match morpheme_prev_kind {
                    Some(..) => "а",
                    None => "я",
                };
                x.to_owned()
            } else {
                match (morpheme_prev_kind, morpheme_next_kind) {
                    // If it is the start of the verb with a negative prefix, the ergative markers get a 'ы'
                    // They basically look like normal absolutive markers
                    (None, Some(MorphemeKind::NegationPrefix))
                    | (None, Some(MorphemeKind::RajImperative)) => {
                        let mut new_marker = marker.clone();
                        new_marker.case = Case::Absolutive;

                        new_marker.get_string()
                    }
                    (None, Some(MorphemeKind::Preverb(..))) if is_raj_imperative => {
                        let x = letter.to_string();

                        let x = x + "ы";
                        if x == "бы" {
                            "у".to_owned()
                        } else {
                            x
                        }
                    }
                    // Howver if there is something before that, like a preverb, the ergative markers behave normally again,
                    // except the second person singular, which stays 'у' instead of becoming 'б'/'п'.
                    (Some(MorphemeKind::Preverb(preverb)), Some(MorphemeKind::NegationPrefix)) => {
                        use ortho::{Manner::*, Place::*};
                        let lc = preverb.get_last_consonant().unwrap();
                        match &letter.kind {
                            ortho::LetterKind::Consonant(consonant) => {
                                let mut co = consonant.clone();
                                co.voiceness = ortho::Voiceness::Voiced;
                                let mut empenthetic = "".to_owned();
                                if (co.place, co.manner) == (Labial, Plosive) {
                                    co.manner = Approximant;
                                    if [Velar, Uvular, Glottal].contains(&lc.place) {
                                        empenthetic = "ы".to_owned();
                                    }
                                }

                                empenthetic + &co.to_string()
                            }
                            ortho::LetterKind::Combi(..) => letter.to_string(),
                            _ => unreachable!(),
                        }
                    }
                    (_, Some(MorphemeKind::Stem(stem))) => match &letter.kind {
                        ortho::LetterKind::Consonant(consonant) => {
                            let mut consonant = consonant.clone();
                            use ortho::Voiceness as OV;
                            use template::ConsonantKind as TCP;
                            consonant.voiceness = match &stem.first_consonant {
                                TCP::Unvoiced => OV::Voiceless,
                                TCP::Voiced => OV::Voiced,
                                _ => unreachable!(),
                            };

                            consonant.to_string()
                        }
                        ortho::LetterKind::Combi(..) => letter.to_string(),
                        _ => unreachable!(),
                    },
                    (None, Some(..)) => match &letter.kind {
                        ortho::LetterKind::Consonant(consonant) => {
                            let mut consonant = consonant.clone();
                            consonant.voiceness = ortho::Voiceness::Voiced;

                            consonant.to_string()
                        }
                        ortho::LetterKind::Combi(..) => letter.to_string(),
                        _ => unreachable!(),
                    },
                    (_, None) => unreachable!(),
                    _ => unreachable!(),
                }
            }
        }
        Case::Absolutive => m.get_base_string(),
        Case::Oblique => unimplemented!(),
    };
    ss
}
fn evaluate_preverb(preverb: &Preverb, morphemes: &VecDeque<Morpheme>, i: usize) -> String {
    let mut p = preverb.clone();
    let morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
    let morpheme_next = morphemes.get(i + 1);
    let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
    let morpheme_next_kind = morpheme_next.map(|x| &x.kind);

    // let mut result = String::new();

    let s = match &morpheme_next.unwrap().kind {
        MorphemeKind::PersonMarker(marker) => {
            assert!(marker.case == Case::Ergative);

            let o = marker.get_base_string();
            let first_letter = ortho::parse(&o)[0].clone();
            match first_letter.kind {
                ortho::LetterKind::Combi(..) => {
                    p.form = PreverbForm::BeforeVowel;
                }
                ortho::LetterKind::Consonant(cons)
                    if cons.base == "б"
                        && morphemes
                            .get(i + 2)
                            .map(|m| m.kind == MorphemeKind::NegationPrefix)
                            .unwrap_or(false) =>
                {
                    p.form = PreverbForm::BeforeVowel
                }
                _ => p.form = PreverbForm::Full,
            }
            preverb.get_string(p.form)
        }
        MorphemeKind::Stem(..) => {
            p.form = PreverbForm::Full;
            preverb.get_string(p.form)
        }
        MorphemeKind::NegationPrefix => {
            p.form = PreverbForm::Full;
            preverb.get_string(p.form)
        }
        MorphemeKind::RajImperative => {
            p.form = PreverbForm::BeforeVowel;
            preverb.get_string(p.form)
        }
        x => unimplemented!("{:?}", x),
    };
    s
}
pub fn evaluate_morphemes(morphemes: &VecDeque<Morpheme>) -> String {
    let mut result = String::new();

    let is_raj_imperative = morphemes.iter().any(|m| {
        if let MorphemeKind::RajImperative = &m.kind {
            return true;
        }
        false
    });

    for (i, morpheme) in morphemes.iter().enumerate() {
        let morpheme_prev = i.checked_sub(1).map(|i| &morphemes[i]);
        let morpheme_next = morphemes.get(i + 1);
        let morpheme_prev_kind = morpheme_prev.map(|x| &x.kind);
        let morpheme_next_kind = morpheme_next.map(|x| &x.kind);

        let _is_first_morpheme = morpheme_prev.is_none();
        let _is_last_morpheme = morpheme_next.is_none();

        match morpheme.kind {
            MorphemeKind::PersonMarker(marker) => {
                result += &evaluate_person_marker(&marker, morphemes, i);
            }
            MorphemeKind::Stem(ref stem) => {
                let tv = template::treat_thematic_vowel(&stem.thematic_vowel, stem);

                let tv = match (&morpheme_prev_kind, &morpheme_next_kind) {
                    (None, None) => tv,
                    (Some(MorphemeKind::PersonMarker(marker)), None)
                        if marker.case == Case::Ergative =>
                    {
                        let morpheme_prev_prev = i.checked_sub(2).map(|i| &morphemes[i]);
                        if morpheme_prev_prev.is_none() {
                            tv
                        } else {
                            "".to_owned()
                        }
                    }
                    _ => "".to_owned(),
                };

                result += &(morpheme.base.clone() + &tv);
            }
            MorphemeKind::Preverb(ref preverb) => {
                result += &evaluate_preverb(&preverb, morphemes, i);
            }
            MorphemeKind::NegationPrefix => {
                result += "мы";
            }
            MorphemeKind::Generic | MorphemeKind::RajImperative => {
                result += &morpheme.base;
            } // _ => unimplemented!(),
        }
    }
    result
}
