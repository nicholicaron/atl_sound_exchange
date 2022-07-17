// https://www.musicgenreslist.com/
// https://stackoverflow.com/questions/55032236/compare-nested-enum-variants-in-rust


#[derive(PartialEq)]
pub enum Genre{
    AlternativeSubgenre,
    BluesSubgenre,
    ClassicalSubgenre,
    CountrySubgenre,
    DanceSubgenre,
    EasyListeningSubgenre,
    ElectronicSubgenre,
    FolkSubgenre,
    HipHopSubgenre,
    HolidaySubgenre,
    IndustrialSubgenre,
    JazzSubgenre,
    LatinSubgenre,
    MetalSubgenre,
    NewAgeSubgenre,
    PopSubgenre,
    RnbSubgenre,
    ReggaeSubgenre,
    RockSubgenre,
    TejanoSubgenre,
    VocalSubgenre,
}

#[derive(PartialEq)]
enum AlternativeSubgenre{
    Alternative,
}

#[derive(PartialEq)]
enum BluesSubgenre{
    Blues,
}

#[derive(PartialEq)]
enum ClassicalSubgenre{
    Classical,
}

#[derive(PartialEq)]
enum CountrySubgenre{
    Country,
}

#[derive(PartialEq)]
enum DanceSubgenre{
    Dance,
}

#[derive(PartialEq)]
enum EasyListeningSubgenre{
    EasyListening,
}

#[derive(PartialEq)]
enum ElectronicSubgenre{
    Electronic,
}

#[derive(PartialEq)]
enum FolkSubgenre{
    Folk,
}

#[derive(PartialEq)]
enum HipHopSubgenre{
    HipHop,
}

#[derive(PartialEq)]
enum HolidaySubgenre{
    Holiday,
}

#[derive(PartialEq)]
enum IndustrialSubgenre{
    Industrial,
}

#[derive(PartialEq)]
enum JazzSubgenre{
    Jazz,
}

#[derive(PartialEq)]
enum LatinSubgenre{
    Latin,
}

#[derive(PartialEq)]
enum MetalSubgenre{
    Metal,
}

#[derive(PartialEq)]
enum NewAgeSubgenre{
    NewAge,
}

#[derive(PartialEq)]
enum PopSubgenre{
    Pop,
}

#[derive(PartialEq)]
enum RnbSubgenre{
    Rnb,
}

#[derive(PartialEq)]
enum ReggaeSubgenre{
    Reggae,
}

#[derive(PartialEq)]
enum RockSubgenre{
    Rock,
}

#[derive(PartialEq)]
enum TejanoSubgenre{
    Tejano,
}

#[derive(PartialEq)]
enum VocalSubgenre{
    Vocal,
}

