// https://www.musicgenreslist.com/
// Consider eventually creating nested enums for subgenres
// https://stackoverflow.com/questions/55032236/compare-nested-enum-variants-in-rust

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub enum Genre {
    Alternative,
    Blues,
    Classical,
    Country,
    Dance,
    EasyListening,
    Electronic,
    Folk,
    HipHop,
    Holiday,
    Industrial,
    Jazz,
    Latin,
    Metal,
    NewAge,
    Pop,
    Rnb,
    Reggae,
    Rock,
    Tejano,
    Vocal,
}

// #[derive(PartialEq)]
// pub enum AlternativeSubgenre{
//     Alternative,
// }

// #[derive(PartialEq)]
// pub enum BluesSubgenre{
//     Blues,
// }

// #[derive(PartialEq)]
// pub enum ClassicalSubgenre{
//     Classical,
// }

// #[derive(PartialEq)]
// pub enum CountrySubgenre{
//     Country,
// }

// #[derive(PartialEq)]
// pub enum DanceSubgenre{
//     Dance,
// }

// #[derive(PartialEq)]
// pub enum EasyListeningSubgenre{
//     EasyListening,
// }

// #[derive(PartialEq)]
// pub enum ElectronicSubgenre{
//     Electronic,
// }

// #[derive(PartialEq)]
// pub enum FolkSubgenre{
//     Folk,
// }

// #[derive(PartialEq)]
// pub enum HipHopSubgenre{
//     HipHop,
// }

// #[derive(PartialEq)]
// pub enum HolidaySubgenre{
//     Holiday,
// }

// #[derive(PartialEq)]
// pub enum IndustrialSubgenre{
//     Industrial,
// }

// #[derive(PartialEq)]
// pub enum JazzSubgenre{
//     Jazz,
// }

// #[derive(PartialEq)]
// pub enum LatinSubgenre{
//     Latin,
// }

// #[derive(PartialEq)]
// pub enum MetalSubgenre{
//     Metal,
// }

// #[derive(PartialEq)]
// pub enum NewAgeSubgenre{
//     NewAge,
// }

// #[derive(PartialEq)]
// pub enum PopSubgenre{
//     Pop,
// }

// #[derive(PartialEq)]
// pub enum RnbSubgenre{
//     Rnb,
// }

// #[derive(PartialEq)]
// pub enum ReggaeSubgenre{
//     Reggae,
// }

// #[derive(PartialEq)]
// pub enum RockSubgenre{
//     Rock,
// }

// #[derive(PartialEq)]
// pub enum TejanoSubgenre{
//     Tejano,
// }

// #[derive(PartialEq)]
// pub enum VocalSubgenre{
//     Vocal,
// }
