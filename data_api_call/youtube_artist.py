import requests
import sys
import csv

api_token = sys.argv[1] 
id = sys.argv[2] 
artist_name = sys.argv[3]
source = "youtube_artist"
filename = source+'.csv'

url = "https://api.chartmetric.com/api/artist/%s/stat/%s/"%(id, source)

headers = {
        'Authorization' : 'Bearer %s'%api_token
        }

response = requests.get(url, params = {'since':'2017-01-01', 'interpolated':'true'}, headers=headers)

if response:
    f = open(r"../../../dev/projects/ATLSE/artist_data/artists/%s/%s"%(artist_name, filename), "a")
    #f = open(filename, "a")
    split_response = response.text.split(",")
    for row in split_response:
        f.write(row + ",\n")    
    f.close()    
else: 
    print('[{}] Error occurred with artist: {} (id: {})'.format(source, artist_name, id))
