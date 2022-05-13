import requests
import json
from datetime import datetime
import sqlite3

# Loading the data of items of interest from a file
# Data is manually added to the file
# Could change to a database as it gets bigger or needs better selection methods
f = open('materials.json')
materials = json.load(f)
f.close()

# Selecting the "listings" endpoint
base_url = "https://api.guildwars2.com/v2"
endpoint_selected = "commerce/listings"

# Connecting to the local database
conn = sqlite3.connect('material_listings.db')
cur = conn.cursor()

# Requesting all items at conce
desired_ids = ""

for item in materials:
    desired_ids = desired_ids + item['id'] + ","
full_endpoint_path = base_url + "/" + endpoint_selected + "?ids="+desired_ids

response = requests.get(full_endpoint_path)
# Considers the difference between requesting and getting the current time negligible
request_time = datetime.now()
request_timestamp = int(request_time.timestamp())
request_time = request_time.isoformat()
current_listings = json.loads(response.content)

for requested_item in current_listings:
    item_to_insert = (request_time, request_timestamp, requested_item['id'],
                      requested_item['buys'][0]['unit_price'], requested_item['buys'][0]['listings'], requested_item['buys'][0]['quantity'],
                      requested_item['sells'][0]['unit_price'], requested_item['sells'][0]['listings'], requested_item['sells'][0]['quantity'])
    sql_insert_item = ''' INSERT INTO listings(request_time,request_timestamp,item_id,buy_price,buy_number_of_listings,buy_quantity,sell_price,sell_number_of_listings,sell_quantity)
                 VALUES(?,?,?,?,?,?,?,?,?) '''
    cur = conn.cursor()
    cur.execute(sql_insert_item, item_to_insert)
    conn.commit()

# Old way, requesting each item separately
# for item in materials.items():
#
#     desired_id = item[1]['id']
#
#     full_endpoint_path = base_url + "/" + endpoint_selected + "?ids=" + desired_id
#
#     response = requests.get(full_endpoint_path)
#     # Considers the difference between requesting and getting the current time negligible
#     request_time = datetime.datetime.now()
#     request_timestamp = request_time.timestamp()
#     current_listings = json.loads(response.content)
#     current_listings = current_listings[0]
#     current_listings['timestamp'] = request_timestamp
#     current_listings['request_time'] = request_time
#
#     # Adding the values to the database
#     item_to_insert = (current_listings['request_time'],current_listings['timestamp'],current_listings['id'],
#                       current_listings['buys'][0]['unit_price'],current_listings['buys'][0]['listings'],current_listings['buys'][0]['quantity'],
#                       current_listings['sells'][0]['unit_price'],current_listings['sells'][0]['listings'],current_listings['sells'][0]['quantity'])
#     sql_insert_item = ''' INSERT INTO listings(request_time,request_timestamp,item_id,buy_price,buy_number_of_listings,buy_quantity,sell_price,sell_number_of_listings,sell_quantity)
#                  VALUES(?,?,?,?,?,?,?,?,?) '''
#     cur = conn.cursor()
#     cur.execute(sql_insert_item, item_to_insert)
#     conn.commit()