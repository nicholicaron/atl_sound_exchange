// Artist Struct
//      Background (Struct)
//          Name
//          Place of Origin (Location(String))
//          Genre(s)
//          description (String)
//          Top songs [Song; 10]
//      deezer (CSV)
//      instagram (CSV)
//      soundcloud (CSV)
//      spotify (CSV)
//      tiktok (CSV)
//      twitter (CSV)
//      Youtube (Struct)
//          yt_artist
//          yt_channel
//
// 
// Song Struct
//      Title
//      Link
// Genre Enum
// Origin Enum
//
// CSV over JSON, because it's more bandwidth friendly as you don't have
// to parse out the syntax, just separate by character
//
//
// Cannot nest structs because size must be known at compile time
// Instead look into RC/ARC 
//      Rc and Arc treat their contents as immutable, if you need mutation,
//      you need to combine them with something (Rc: RefCell/ Arc: Mutex)
