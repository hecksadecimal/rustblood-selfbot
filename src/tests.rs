#[cfg(test)]
mod quirks {
    use crate::quirk::{self, Characters};
    use crate::quirk::Character;
    use test_case::test_case;

    #[test_case(false ; "without unicode")]
    #[test_case(true ; "with unicode")]
    fn can_serialize_deserialize(unicode: bool) {
        let data;
        if !unicode {
            data = r#"
            // 'quirks' is an array of dicts that describes operations to be done on a string. Operations are done in the order these quirks are described in.
            // 'handle' is just a convenience for the script to display for debugging.
            // 'acronym' is used to construct the final message. Allows for characters with the same acronym to be used, just with different filenames to reference them. Optional.
            //
            // Possible quirks:
            //                  prefix:
            //                  Adds the following characters to the beginning of the string.
            //
            //                  suffix:
            //                  Adds the following characters to the end of the string.
            //
            //                  simple_replacements:
            //                  For each replacement operation described, changes all occurances of the supplied character sequences inside the string.
            //                  Operations occur in their described order.
            //
            //                  random_replacements:
            //                  For each replacement operation described, replaces all occurances of the supplied character sequence inside the string with a random selection.
            //                  Operations occut in their described order.
            //
            //                  scramble:
            //                  For each operation described, changes all occurances of the supplied character sequence inside the string with a random arrangement of the characters supplied.
            //                  Operations occur in their described order.
            //
            //                  style:
            //                  Describes a typing style. If this is specified, the script will attempt to apply special operations.
            //                  Supported values: lowercase, uppercase, alternating, camelcase, reverse, inverted
            //
            //                  WARNING!!! Make sure to only include the following structure in your quirk file. Including these comments inside the file will render it inoperable.
            {
                "handle": "yourHandle",
                "acronym": "YH",
                "quirks": [
                    {
                        "prefix": ">>> "
                    },
                    {
                        "simple_replacements": [
                            [
                                "z", "zee"
                            ]
                        ]
                    },
                    {
                        "random_replacements": [
                            [
                                "lmao", ["laughing my ass off", "assing my laugh off", "hahahaha"] // lol
                            ]
                        ]
                    },
                    {
                        "scramble": [
                            [
                                "fuck", "%&#@"
                            ],
                            [
                                "shit", "%&!@"
                            ],
                            [
                                "ass", "@$%"
                            ]
                        ]
                    },
                    {
                        "suffix": " <<<"
                    },
                    {
                        "style": "lowercase"
                    }
                ]
            }
            "#;
        } else {
            data = r#"
            {
                // Comment
                // Test goes
                // Here
                "handle": "lowhungNoose",
                "acronym": "LN",
                "quirks": [
                    {
                        "prefix": "C>lll= "
                    },
                    {
                        "simple_replacements": [
                            [
                                "A", "Д"
                            ],
                            [
                                "a", "д"
                            ],
                            [
                                "E", "Э"
                            ],
                            [
                                "e", "э"
                            ],
                            [
                                "B", "В"
                            ],
                            [
                                "b", "ѣ"
                            ],
                            [
                                "r","г"
                            ],
                            [
                                "R", "Я"
                            ],
                            [
                                "W", "Ш"
                            ],
                            [
                                "T", "Т"
                            ],
                            [
                                "t", "т"
                            ],
                            [
                                "O", "Ø"
                            ],
                            [
                                "V", "Ѵ"
                            ],
                            [
                                "v", "ѵ"
                            ],
                            [
                                "io", "Ю"
                            ],
                            [
                                "X", "Ж"
                            ],
                            [
                                "x", "ж"
                            ],
                            [
                                "N", "И"
                            ],
                            [
                                "n", "л"
                            ],
                            [
                                "m", "м"
                            ],
                            [
                                "Y", "Ч"
                            ],
                            [ 
                                "y", "ч"
                            ]
                        ]
                    },
                    {
                        "suffix": " ==" // Comment test
                    }
                ]
            }
            "#;
        }
        
    
        let c = quirk::parse_safe(data.to_string());
    
        let j = serde_json::to_string_pretty(&c).unwrap();
    
        assert!(c.quirks.len() > 0); // Our quirks vec actually has content
        if unicode {
            assert!(c.quirks[0]["prefix"] == "C>lll= "); // The first element of quirks is what we expect
            assert!(c.quirks[2]["suffix"] == " =="); // The last element of quirks is what we expect
        } else {
            assert!(c.quirks[0]["prefix"] == ">>> "); // The first element of quirks is what we expect
            assert!(c.quirks[5]["style"] == "lowercase"); // The last element of quirks is what we expect
        }
        assert!(quirk::parse_safe(j) == c); // The values did not mutate for no reason
    
    }
    
    #[test_case(">>> " ; "without unicode")]
    #[test_case("𝖟∞ " ; "with unicode")]
    fn can_prefix(s: &str) {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "prefix": ""
                }
            ]
        }
        "#;
        let data = data.replace("\"\"", &format!("\"{}\"", s));
        let c = quirk::parse_safe(data.to_string());
        let test_string = "The quick brown fox jumped over the lazy dog.";
    
        let mutated_string = c.quirked(test_string);
    
        assert!(mutated_string == format!("YH: {}The quick brown fox jumped over the lazy dog.", s));
    }
    
    #[test_case(" <<<" ; "without unicode")]
    #[test_case(" 𝖟∞" ; "with unicode")]
    fn can_suffix(s: &str) {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "suffix": ""
                }
            ]
        }
        "#;
        let data = data.replace("\"\"", &format!("\"{}\"", s));

        let c = quirk::parse_safe(data.to_string());
        let test_string = "The quick brown fox jumped over the lazy dog.";
    
        let mutated_string = c.quirked(test_string);
    
        assert!(mutated_string == format!("YH: The quick brown fox jumped over the lazy dog.{}", s));
    }
    
    #[test_case("zee" ; "without unicode")]
    #[test_case("z𝖟𝖟" ; "with unicode")]
    fn can_simple_replace(s: &str) {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "simple_replacements": [
                        [
                            "z", ""
                        ],
                        [
                            "The", "Da"
                        ]
                    ]
                }
            ]
        }
        "#;
        let data = data.replace("\"\"", &format!("\"{}\"", s));

        let c = quirk::parse_safe(data.to_string());
        let test_string = "The quick brown fox jumped over the lazy dog.";
    
        let mutated_string = c.quirked(test_string);
        
        assert!(mutated_string == format!("YH: Da quick brown fox jumped over the la{}y dog.", s));
    }
    
    #[test_case("red", "blue", "green" ; "without unicode")]
    #[test_case("𝓻𝖟𝓭", "𝓫𝓵𝓊𝓮", "𝓰𝓻𝓮𝓮𝓷" ; "with unicode")]
    fn can_random_replace(s1: &str, s2: &str, s3: &str) {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "random_replacements": [
                        [
                            "brown", ["s1", "s2", "s3"]
                        ]
                    ]
                }
            ]
        }
        "#;

        let data = data.replace("\"s1\"", &format!("\"{}\"", s1));
        let data = data.replace("\"s2\"", &format!("\"{}\"", s2));
        let data = data.replace("\"s3\"", &format!("\"{}\"", s3));


        let c = quirk::parse_safe(data.to_string());
        let test_string = "The quick brown fox jumped over the lazy dog.";
    
        let mutated_string = c.quirked(test_string);
    
        assert!(mutated_string == format!("YH: The quick {} fox jumped over the lazy dog.", s1) ||
                mutated_string == format!("YH: The quick {} fox jumped over the lazy dog.", s2) ||
                mutated_string == format!("YH: The quick {} fox jumped over the lazy dog.", s3));
    }
    
    #[test_case("(?P<y>\\\\d{4})-(?P<m>\\\\d{2})-(?P<d>\\\\d{2})", "$m/$d/$y" ; "without unicode")]
    #[test_case("(?P<y>\\\\d{4})-(?P<m>\\\\d{2})-(?P<d>\\\\d{2})", "$m𝓵$d𝓵$y" ; "with unicode")]
    fn can_regex_replace(s1: &str, s2: &str) {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "regex_replacements": [
                        [
                            "s1", "s2"
                        ]
                    ]
                }
            ]
        }
        "#;

        let data = data.replace("\"s1\"", &format!("\"{}\"", s1));
        let data = data.replace("\"s2\"", &format!("\"{}\"", s2));

        let c = quirk::parse_safe(data.to_string());
        let test_string = "The quick 2014-01-01 brown fox jumped over the lazy dog.";
    
        let mutated_string = c.quirked(test_string);
    
        assert!(mutated_string == "YH: The quick 01/01/2014 brown fox jumped over the lazy dog." ||
                mutated_string == "YH: The quick 01𝓵01𝓵2014 brown fox jumped over the lazy dog.");
    }

    #[test_case("red", "blue", "green", false ; "without unicode")]
    #[test_case("𝓻𝖟𝓭", "𝓫𝓵𝓊𝓮", "𝓰𝓻𝓮𝓮𝓷", true ; "with unicode")]
    fn can_scramble(s1: &str, s2: &str, s3: &str, unicode: bool) {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "scramble": [
                        [
                            "quick", "s1"
                        ],
                        [
                            "jumped", "s2"
                        ],
                        [
                            "lazy", "s3"
                        ]
                    ]
                }
            ]
        }
        "#;
        let data = data.replace("\"s1\"", &format!("\"{}\"", s1));
        let data = data.replace("\"s2\"", &format!("\"{}\"", s2));
        let data = data.replace("\"s3\"", &format!("\"{}\"", s3));

        let c = quirk::parse_safe(data.to_string());
        let test_string = "The quick brown fox jumped over the lazy dog.";
    
        let mutated_string = c.quirked(test_string);
        
        assert!(!(mutated_string.contains("quick")) && !(mutated_string.contains("jumped") && !(mutated_string.contains("lazy"))));
        if unicode {
            assert!(mutated_string.contains("𝖟") || mutated_string.contains("𝓵") || mutated_string.contains("𝓰"));
        } 
    }
    
    #[test_case("The quick brown fox jumped over the lazy dog." ; "without unicode")]
    #[test_case("Тhe quick brown fox jumped over the lazy dog." ; "with unicode")]
    fn can_lowercase_style(s: &str) {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "style": "lowercase"
                }
            ]
        }
        "#;
        let c = quirk::parse_safe(data.to_string());
        let test_string = format!("{}", s);
    
        let mutated_string = c.quirked(test_string.as_str());
        
        assert!(mutated_string == "YH: the quick brown fox jumped over the lazy dog." ||
                mutated_string == "YH: тhe quick brown fox jumped over the lazy dog.");    
    }
    
    #[test_case("The quick brown fox jumped over the lazy dog." ; "without unicode")]
    #[test_case("ThЭ quick brown fox jumped over the lazy dog." ; "with unicode")]
    fn can_uppercase_style(s: &str) {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "style": "uppercase"
                }
            ]
        }
        "#;
        let c = quirk::parse_safe(data.to_string());
        let test_string = format!("{}", s);
    
        let mutated_string = c.quirked(test_string.as_str());
        
        assert!(mutated_string == "YH: THE QUICK BROWN FOX JUMPED OVER THE LAZY DOG." ||
                mutated_string == "YH: THЭ QUICK BROWN FOX JUMPED OVER THE LAZY DOG."); 
    }

    #[test_case("The quick brown fox jumped over the lazy dog." ; "without unicode")]
    #[test_case("ThЭ quick brown fox jumped over the lazy dog." ; "with unicode")]
    fn can_alternating_style(s: &str) {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "style": "alternating"
                }
            ]
        }
        "#;
        let c = quirk::parse_safe(data.to_string());
        let test_string = format!("{}", s);
    
        let mutated_string = c.quirked(test_string.as_str());
        
        assert!(mutated_string == "YH: ThE QuIcK BrOwN FoX JuMpEd oVeR ThE LaZy dOg." ||
                mutated_string == "YH: ThЭ QuIcK BrOwN FoX JuMpEd oVeR ThE LaZy dOg."); 
    }

    #[test_case("The quick brown fox jumped over the lazy dog." ; "without unicode")]
    #[test_case("ThЭ quick brown fox jumped over the lazy dog." ; "with unicode")]
    fn can_camelcase_style(s: &str) {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "style": "camelcase"
                }
            ]
        }
        "#;
        let c = quirk::parse_safe(data.to_string());
        let test_string = format!("{}", s);
    
        let mutated_string = c.quirked(test_string.as_str());
        
        assert!(mutated_string == "YH: The Quick Brown Fox Jumped Over The Lazy Dog." ||
                mutated_string == "YH: ThЭ Quick Brown Fox Jumped Over The Lazy Dog."); 
    }

    #[test_case("The quick brown fox jumped over the lazy dog." ; "without unicode")]
    #[test_case("ThЭ quick brown fox jumped over the lazy dog." ; "with unicode")]
    fn can_reverse_style(s: &str) {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "style": "reverse"
                }
            ]
        }
        "#;
        let c = quirk::parse_safe(data.to_string());
        let test_string = format!("{}", s);
    
        let mutated_string = c.quirked(test_string.as_str());
        
        assert!(mutated_string == "YH: .god yzal eht revo depmuj xof nworb kciuq ehT" ||
                mutated_string == "YH: .god yzal eht revo depmuj xof nworb kciuq ЭhT"); 
    }

    #[test_case("The quick brown fox jumped over the lazy dog." ; "without unicode")]
    #[test_case("ThЭ quick brown fox jumped over the lazy dog." ; "with unicode")]
    fn can_inverted_style(s: &str) {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "style": "inverted"
                }
            ]
        }
        "#;
        let c = quirk::parse_safe(data.to_string());
        let test_string = format!("{}", s);
    
        let mutated_string = c.quirked(test_string.as_str());
        
        assert!(mutated_string == "YH: tHE QUICK BROWN FOX JUMPED OVER THE LAZY DOG." ||
                mutated_string == "YH: tHэ QUICK BROWN FOX JUMPED OVER THE LAZY DOG."); 
    }

    #[test]
    fn can_combine_quirks() {
        let data = r#"
        {
            "handle": "yourHandle",
            "acronym": "YH",
            "quirks": [
                {
                    "style": "reverse"
                },
                {
                    "style": "uppercase"
                }
            ]
        }
        "#;
        let c = quirk::parse_safe(data.to_string());
        let test_string = "The quick brown fox jumped over the lazy dog.";
    
        let mutated_string = c.quirked(test_string);
        
        assert!(mutated_string == "YH: .GOD YZAL EHT REVO DEPMUJ XOF NWORB KCIUQ EHT"); 
    }

    #[test_case( "" ; "Trial one")]
    #[test_case( "" ; "Trial two")]
    #[test_case( "" ; "Trial three")]
    #[test_case( "" ; "Trial four")]
    #[test_case( "" ; "Trial five")]
    fn quirks_are_deterministic(_s: &str) {
        let data = r#"
        {
            "handle": "syntaxError",
            "acronym": "`SE",
            "quirks": [
                {
                    "style": "lowercase"
                },
                {
                    "prefix": ">_ "
                },
                {
                    "suffix": "`"
                },
                {
                    "simple_replacements": [
                        [
                            "lorem", "[<censor>]"
                        ],
                        [
                            "ipsum", "[<censor>]"
                        ],
                        [
                            "dolar", "[<censor>]"
                        ],
                        [
                            "somet", "[<mini-censor>]"
                        ]
                    ]
                },
                {
                    "simple_replacements": [
                        [
                            "<mini-censor>", "▜▟"
                        ],
                        [
                            "<censor>", "▛▜▟▙"
                        ]
                    ]
                },
                {
                    "simple_replacements": [
                        [
                            ".", ""
                        ],
                        [
                            ",", ""
                        ],
                        [
                            "'", ""
                        ],
                        [
                            "ing", "in"
                        ],
                        [
                            "l", "1"
                        ],
                        [
                            "i", "1"
                        ],
                        [
                            "o", "0"
                        ]
                    ]
                }
            ]
        }
        "#;
        let c = quirk::parse_safe(data.to_string());
        let test_string = "The lorem dolar fox jumped ipsum the somet dog.";
    
        let mutated_string = c.quirked(test_string);
        
        assert!(mutated_string == "`SE: >_ the [▛▜▟▙] [▛▜▟▙] f0x jumped [▛▜▟▙] the [▜▟] d0g`");
    }

    #[test]
    fn can_get_character_from_name() {
        let c = Character::from_name("ARADIA");
        if !c.is_some() {
            panic!("Character quirk file could not be loaded.")
        }
    }

    #[test]
    fn cant_get_character_from_invalid_name() {
        let c = Character::from_name("MISSING");
        if c.is_some() {
            panic!("Missing character quirk file should not return a character")
        }
    }

    #[test]
    fn can_quirk_multiline() {
        let string = r#"ARADIA: Lorem ipsum dolar somet.
NOTRADIA: Nothing to see here."#;

        let expected_string = r#"AA: lorem ipsum dolar somet
NOTRADIA: Nothing to see here."#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }

    #[test]
    fn can_join_memo() {
        let string = "ARADIA! join";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] has joined the memo! --
```"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }


    #[test]
    fn can_leave_memo() {
        let string = "ARADIA! leave";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] has left the memo! --
```"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }

    #[test]
    fn can_go_online() {
        let string = "ARADIA! online";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] is now online! --
```"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }

    #[test]
    fn can_go_offline() {
        let string = "ARADIA! offline";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] is now offline! --
```"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }

    #[test]
    fn can_go_idle() {
        let string = "ARADIA! idle";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] is now idle! --
```"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }

    #[test]
    fn can_go_unidle() {
        let string = "ARADIA! unidle";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] is no longer idle! --
```"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }

    #[test]
    fn can_block() {
        let string = "ARADIA! block ectoBiologist";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] has blocked ectoBiologist [EB]! --
```
"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }

    #[test]
    fn can_unblock() {
        let string = "ARADIA! unblock ectoBiologist";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] has unblocked ectoBiologist [EB]! --
```
"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }

    #[test]
    fn can_upload() {
        let string = "ARADIA! upload test.png";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] has uploaded "test.png" --
```
"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }

    #[test]
    fn can_kick() {
        let string = "ARADIA! kick ectoBiologist";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] has kicked ectoBiologist [EB] from the memo! --
```
"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }

    #[test]
    fn can_ban() {
        let string = "ARADIA! ban ectoBiologist";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] has banned ectoBiologist [EB] from the memo! --
```
"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }

    #[test]
    fn can_unban() {
        let string = "ARADIA! unban ectoBiologist";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] has unbanned ectoBiologist [EB] from the memo! --
```
"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }

    #[test]
    fn can_troll() {
        let string = "ARADIA! troll ectoBiologist";
        let expected_string = 
r#"```
-- apocalypseArisen [AA] has begun trolling ectoBiologist [EB]! --
```
"#;

        let cs = Characters::from_string(string);
        let quirked_text = cs.quirked();
        assert_eq!(quirked_text, expected_string);
    }
}

