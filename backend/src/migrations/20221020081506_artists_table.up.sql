-- Add up migration script here
CREATE TABLE IF NOT EXISTS artists (
  id serial PRIMARY KEY,
  artist_name VARCHAR (255) NOT NULL,
  genre VARCHAR (255),
  socials TEXT [],
  background TEXT [],
  description TEXT,
  deezer JSONB,
  instagram JSONB,
  soundcloud JSONB,
  spotify JSONB,
  tiktok JSONB,
  twitter JSONB,
  yt_channel JSONB,
  yt_artist JSONB
);



/* Artist DB schema prototype:

   Artists
   - Artist 1
   + id: int4
   + name: varchar
   + genre: varchar
   + socials (varchar, varchar, varchar, varchar)
   + Background
 * Origin
   - city: varchar
   - state: char[2]
   - country: char[2]
 * Description: varchar
   + deezer: jsonb
   + instagram: jsonb
   + soundcloud: jsonb
   + spotify: jsonb
   + tiktok: jsonb
   + twitter: jsonb
   + yt_channel: jsonb
   + yt_artist: jsonb

   Why Jsonb?
   Jsonb is stored in a decomposed binary format
   - Slightly slower input due to conversion overhead
   - Significantly faster to process, since no reparsing needed
   - Also supports indexing
 */
