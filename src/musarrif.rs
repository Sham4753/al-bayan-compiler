#[derive(Debug, Clone)]
pub struct Musarrafa {
    pub jidhr: String,
    pub wazn: String,
    pub zaman: Zaman,
    pub damair: Vec<Damir>,
    pub irab: Option<Irab>,
    pub original: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zaman { Madin, Mudari3, Amr, Mustaqbal }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Damir {
    Ana, Anta, Anti, Huwa, Hiya, Nahnu, Antum, Hum,
    Ni, Ka, Ki, Hu, Ha, Na, Kum, HumPronoun,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Irab { Raf3, Nasb, Jarr, Jazm }

pub struct Musarrif;

impl Musarrif {
    pub fn analyse(input: &str) -> Result<Musarrafa, String> {
        let original = input.to_string();

        let mut letters: Vec<char> = Vec::new();
        let mut harakat: Vec<(usize, char)> = Vec::new();

        for c in input.chars() {
            let cv = c as u32;
            if (0x064B..=0x0652).contains(&cv) || cv == 0x0670 {
                harakat.push((letters.len(), c));
            } else {
                letters.push(c);
            }
        }

        if letters.is_empty() {
            return Err("لا توجد حروف".to_string());
        }

        let mut damair = Vec::new();
        let mut zaman = Zaman::Madin;
        let len = letters.len();

        let mut end_cut = 0;
        if len >= 2 {
            let last_two = format!("{}{}", letters[len-2], letters[len-1]);
            match last_two.as_str() {
                "ني" | "نى" => { damair.push(Damir::Ni); end_cut = 2; }
                "كم" => { damair.push(Damir::Kum); end_cut = 2; }
                "هم" => { damair.push(Damir::HumPronoun); end_cut = 2; }
                "نا" => { damair.push(Damir::Na); end_cut = 2; }
                "ها" => { damair.push(Damir::Ha); end_cut = 2; }
                _ => {}
            }
        }
        if end_cut == 0 && len >= 1 {
            match letters[len-1] {
                'ك' => { damair.push(Damir::Ka); end_cut = 1; }
                'ه' => { damair.push(Damir::Hu); end_cut = 1; }
                _ => {}
            }
        }

        let core_len = len - end_cut;

        let mut start_idx = 0;
        if core_len > 1 && letters[0] == 'س' {
            zaman = Zaman::Mustaqbal;
            start_idx = 1;
        }

        if start_idx < core_len {
            match letters[start_idx] {
                'أ' | 'ت' | 'ي' | 'ن' => {
                    if matches!(zaman, Zaman::Madin) { zaman = Zaman::Mudari3; }
                    damair.push(match letters[start_idx] {
                        'أ' => Damir::Ana,
                        'ت' => Damir::Anta,
                        'ي' => Damir::Huwa,
                        'ن' => Damir::Nahnu,
                        _ => unreachable!(),
                    });
                    start_idx += 1;
                }
                _ => {}
            }
        }

        let root_slice = &letters[start_idx..core_len];
        if root_slice.is_empty() {
            return Err("لا توجد حروف جذر".to_string());
        }

        // كشف الشدة
        let has_shadda = harakat.iter().any(|(_, h)| *h == '\u{0651}');

        let (wazn, root_letters_clean) = if has_shadda {
            ("فَعَّلَ".to_string(), root_slice.to_vec())
        } else {
            identify_wazn_and_clean(root_slice)
        };

        let jidhr: String = root_letters_clean.iter().take(3).collect();

        let irab = harakat.iter()
            .filter(|(pos, _)| *pos < core_len && *pos > 0)
            .last()
            .and_then(|(_, h)| match h {
                '\u{064E}' | '\u{064B}' => Some(Irab::Nasb),
                '\u{064F}' | '\u{064C}' => Some(Irab::Raf3),
                '\u{0650}' | '\u{064D}' => Some(Irab::Jarr),
                '\u{0652}' => Some(Irab::Jazm),
                _ => None,
            });

        Ok(Musarrafa {
            jidhr,
            wazn,
            zaman,
            damair,
            irab,
            original,
        })
    }
}

fn identify_wazn_and_clean(slice: &[char]) -> (String, Vec<char>) {
    let len = slice.len();
    if len == 0 { return ("فَعَلَ".to_string(), vec![]); }

    // اِستَفعَلَ
    if len >= 5 && slice[0] == 'ا' && slice[1] == 'س' && slice[2] == 'ت' {
        return ("اِستَفعَلَ".to_string(), slice[3..].to_vec());
    }

    // اِفتَعَلَ
    if len >= 4 && slice[0] == 'ا' && slice[2] == 'ت' {
        let clean: Vec<char> = slice.iter().enumerate()
            .filter(|(i, _)| *i != 0 && *i != 2)
            .map(|(_, &c)| c)
            .collect();
        return ("اِفتَعَلَ".to_string(), clean);
    }

    // اِنفَعَلَ
    if len >= 4 && slice[0] == 'ا' && slice[1] == 'ن' {
        return ("اِنفَعَلَ".to_string(), slice[2..].to_vec());
    }

    // تَفَاعَلَ
    if len >= 4 && slice[0] == 'ت' && slice[2] == 'ا' {
        let clean: Vec<char> = slice.iter().enumerate()
            .filter(|(i, _)| *i != 0 && *i != 2)
            .map(|(_, &c)| c)
            .collect();
        return ("تَفَاعَلَ".to_string(), clean);
    }

    // أمر
    if len >= 1 && slice[0] == 'ا' {
        return ("اِفعَل".to_string(), slice[1..].to_vec());
    }

    // فَاعَلَ
    if len >= 3 && slice[1] == 'ا' {
        let clean: Vec<char> = slice.iter().enumerate()
            .filter(|(i, _)| *i != 1)
            .map(|(_, &c)| c)
            .collect();
        return ("فَاعَلَ".to_string(), clean);
    }

    ("فَعَلَ".to_string(), slice.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qaraa() {
        let r = Musarrif::analyse("قَرَأَ").unwrap();
        assert_eq!(r.jidhr, "قرأ");
        assert_eq!(r.wazn, "فَعَلَ");
        assert_eq!(r.zaman, Zaman::Madin);
    }

    #[test]
    fn test_yuhasibu() {
        let r = Musarrif::analyse("يُحَاسِبُ").unwrap();
        assert_eq!(r.jidhr, "حسب");
        assert_eq!(r.wazn, "فَاعَلَ");
        assert_eq!(r.zaman, Zaman::Mudari3);
    }

    #[test]
    fn test_sayuhassibu() {
        let r = Musarrif::analyse("سَيُحَسِّبُ").unwrap();
        assert_eq!(r.jidhr, "حسب");
        assert_eq!(r.wazn, "فَعَّلَ");
        assert_eq!(r.zaman, Zaman::Mustaqbal);
    }

    #[test]
    fn test_istaqraa() {
        let r = Musarrif::analyse("اِستَقرَأَ").unwrap();
        assert_eq!(r.jidhr, "قرأ");
        assert_eq!(r.wazn, "اِستَفعَلَ");
    }

    #[test]
    fn test_yahfathuhu() {
        let r = Musarrif::analyse("يَحْفَظُهُ").unwrap();
        assert_eq!(r.jidhr, "حفظ");
        assert!(r.damair.iter().any(|d| matches!(d, Damir::Hu)));
    }

    #[test]
    fn test_ihtasaba() {
        let r = Musarrif::analyse("اِحتَسَبَ").unwrap();
        assert_eq!(r.jidhr, "حسب");
        assert_eq!(r.wazn, "اِفتَعَلَ");
    }

    #[test]
    fn test_inba3atha() {
        let r = Musarrif::analyse("اِنبَعَثَ").unwrap();
        assert_eq!(r.jidhr, "بعث");
        assert_eq!(r.wazn, "اِنفَعَلَ");
    }
}
