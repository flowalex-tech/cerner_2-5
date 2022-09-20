import urllib
from urllib.request import urlopen
import json
import pyjq
# cerner_2tothe5th_2022
# Where to find Zone IDs by state: https://www.weather.gov/primar/PubZone
# nwsurl = 'https://api.weather.gov/alerts/active/zone/MNZ060' # Hennipen County
# nwsurl = 'https://api.weather.gov/alerts/active/zone/MNZ021' # Southern Cook/ North Shore Grand Marais
nwsurl = 'https://api.weather.gov/alerts/active/zone/MNZ020' # Southern Cook/ North Shore Two Harbors
# nwsurl = 'https://api.weather.gov/alerts/active/zone/MOZ037' # KCMO

results = pyjq.all(".features[1].properties | .senderName, .headline, .description, .areaDesc, .effective, .expires", url=nwsurl)

if None in results:
    print("No Active Alerts for Region")
else:
    for i in results:
        print(i)
