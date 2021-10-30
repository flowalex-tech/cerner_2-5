import requests
from requests.packages.urllib3.exceptions import InsecureRequestWarning
import pandas as pd
# cerner_2tothe5th_2021
requests.packages.urllib3.disable_warnings(InsecureRequestWarning)

metrotransit_fairview_eb = requests.get('http://svc.metrotransit.org/NexTrip/902/0/FAUN?format=json').json()
metrotransit_fairview_wb = requests.get('http://svc.metrotransit.org/NexTrip/902/1/FAUN?format=json').json()

arrival_list = []

for item in metrotransit_fairview_eb:
    stop_details = {"Description":None,"DepartureText":None,"Route":None,"RouteDirection":None}
    stop_details["Description"] = item['Description']
    stop_details["DepartureText"] = item['DepartureText']
    stop_details["Route"] = item['Route']
    stop_details["RouteDirection"] = item['RouteDirection']
    arrival_list.append(stop_details)

for item in metrotransit_fairview_wb:
    stop_details = {"Description":None,"DepartureText":None,"Route":None,"RouteDirection":None}
    stop_details["Description"] = item['Description']
    stop_details["DepartureText"] = item['DepartureText']
    stop_details["Route"] = item['Route']
    stop_details["RouteDirection"] = item['RouteDirection']
    arrival_list.append(stop_details)

    df = pd.DataFrame(data=arrival_list, columns=['Description','DepartureText','Route','RouteDirection'])
    fairview_df = df.rename(columns={'Description' : 'Destination','DepartureText' : 'Scheduled Arrival','Route' : 'Route','RouteDirection'  : 'Direction'})

    print(fairview_df(20))
