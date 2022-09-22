import spotify
import twitter
import instagram
import youtube_artist
import youtube_channel
import deezer
import soundcloud
import tiktok
import sys

api_token = sys.argv[1]
id = sys.argv[2]
artist_name = sys.argv[3]

spotify(id, artist_name)
twitter(id, artist_name)
instagram(id, artist_name)
youtube(id, artist_name)
deezer(id, artist_name)
pandora(id, artist_name)
soundcloud(id, artist_name)
tiktok(id, artist_name)
print("finished with " + artist_name)
