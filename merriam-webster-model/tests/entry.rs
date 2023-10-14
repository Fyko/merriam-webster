use merriam_webster_model::entry::Entry;

macro_rules! impl_word_test {
    ($func:ident, $word:literal) => {
        #[test]
        fn $func() -> Result<(), anyhow::Error> {
            let path = format!("{}/tests/words/{}.json", env!("CARGO_MANIFEST_DIR"), $word);
            dbg!(path.clone());
            let json = std::fs::read_to_string(path)?;
            let jd = &mut serde_json::Deserializer::from_str(&json);

            let result: Result<Vec<Entry>, _> = serde_path_to_error::deserialize(jd);
            match result {
                Ok(_) => assert!(true),
                Err(err) => {
                    let message = err.to_string();
                    panic!("{message}");
                }
            };

            Ok(())
        }
    };
}

impl_word_test!(test_word_aesthetic, "aesthetic");
impl_word_test!(test_word_ally, "ally");
impl_word_test!(test_word_alpha, "alpha");
impl_word_test!(test_word_ane, "ane");
impl_word_test!(test_word_annular, "annular");
impl_word_test!(test_word_an, "an");
impl_word_test!(test_word_balls, "balls");
impl_word_test!(test_word_banana, "banana");
impl_word_test!(test_word_bolster, "bolster");
impl_word_test!(test_word_cardi, "cardi");
impl_word_test!(test_word_chat, "chat");
impl_word_test!(test_word_constitution, "constitution");
impl_word_test!(test_word_creator, "creator");
impl_word_test!(test_word_dislimned, "dislimned");
impl_word_test!(test_word_dude, "dude");
impl_word_test!(test_word_emulate, "emulate");
impl_word_test!(test_word_essentialism, "essentialism");
impl_word_test!(test_word_fall_foul_of, "fall_foul_of");
impl_word_test!(test_word_faucet, "faucet");
impl_word_test!(test_word_fava, "fava");
impl_word_test!(test_word_featherweight, "featherweight");
impl_word_test!(test_word_fer, "-fer");
impl_word_test!(test_word_follow, "follow");
impl_word_test!(test_word_gaslighting, "gaslighting");
impl_word_test!(test_word_grandmother, "grandmother");
impl_word_test!(test_word_hoo, "hoo");
impl_word_test!(test_word_humble, "humble");
impl_word_test!(test_word_joint, "joint");
impl_word_test!(test_word_justice, "justice");
impl_word_test!(test_word_limerick, "limerick");
impl_word_test!(test_word_lincoln, "Lincoln");
impl_word_test!(test_word_mascara, "mascara");
impl_word_test!(test_word_mean, "mean");
impl_word_test!(test_word_mendelism, "mendelism");
impl_word_test!(test_word_metabolism, "metabolism");
impl_word_test!(test_word_milf, "milf");
impl_word_test!(test_word_mush, "mush");
impl_word_test!(test_word_nuance, "nuance");
impl_word_test!(test_word_pronoun, "pronoun");
impl_word_test!(test_word_random, "random");
impl_word_test!(test_word_rapper, "rapper");
impl_word_test!(test_word_resolution, "resolution");
impl_word_test!(test_word_rizz, "rizz");
impl_word_test!(test_word_silly, "silly");
impl_word_test!(test_word_stallion, "stallion");
impl_word_test!(test_word_synonym, "synonym");
impl_word_test!(test_word_translate, "translate");
impl_word_test!(test_word_tube, "tube");
impl_word_test!(test_word_vagary, "vagary");
impl_word_test!(test_word_voracity, "voracity");
impl_word_test!(test_word_with, "with");
impl_word_test!(test_word_yeet, "yeet");
impl_word_test!(test_word_zenith, "zenith");
