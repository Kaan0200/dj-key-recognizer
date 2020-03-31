pub mod DJKey {

    struct MusicalKey {
          c_key: CamelotKey,
    }


    impl MusicalKey {
        fn from_camelot(input_key: CamelotKey) -> MusicalKey {
            MusicalKey { c_key: input_key }
        }

        fn from_raw_as_camelot(input_raw: &str) -> MusicalKey {
            //must be length of two or three
            if input_raw.len() < 4 || input_raw.len() > 1 {
                //convert to camelot
                let new_key = match input_raw.parse::<CamelotKey>() {
                    Ok(key) => key,
                    Err(e) => panic!(e),
                };

                return MusicalKey {
                    c_key: new_key,
                };
            }

            panic!("Key length is invalid, input was {} long", input_raw.len());
        }
        
    }

    pub enum CamelotKey {
        A1,B1,
        A2,B2,
        A3,B3,
        A4,B4,
        A5,B6,
        A6,B5,
        A7,B7,
        A8,B8,
        A9,B9,
        A10,B10,
        A11,B11,
        A12,B12
    }

    impl std::str::FromStr for CamelotKey {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A1" => Ok(CamelotKey::A1),"B1" => Ok(CamelotKey::B1),
                "A2" => Ok(CamelotKey::A2),"B2" => Ok(CamelotKey::B2),
                "A3" => Ok(CamelotKey::A3),"B3" => Ok(CamelotKey::B3),
                "A4" => Ok(CamelotKey::A4),"B4" => Ok(CamelotKey::B4),
                "A5" => Ok(CamelotKey::A5),"B5" => Ok(CamelotKey::B5),
                "A6" => Ok(CamelotKey::A6),"B6" => Ok(CamelotKey::B6),

                "A7" => Ok(CamelotKey::A7),"B7" => Ok(CamelotKey::B7),
                "A8" => Ok(CamelotKey::A8),"B8" => Ok(CamelotKey::B8),
                "A9" => Ok(CamelotKey::A9),"B9" => Ok(CamelotKey::B9),
                "A10" => Ok(CamelotKey::A10),"B10" => Ok(CamelotKey::B10),
                "A11" => Ok(CamelotKey::A11),"B11" => Ok(CamelotKey::B11),
                "A12" => Ok(CamelotKey::A12),"B12" => Ok(CamelotKey::B12),
                _ => Err(format!("'{}' is not a valid Camelot Key value", s)),
            }
        }
    }

    enum Key {
        A, B, C, D, E, F, G
    }
}
