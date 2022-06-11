import json
import requests

f = open('materials.json')
materials = json.load(f)
f.close()

base_url = "http://api.gw2tp.com"
endpoint_selected = "/1/items"

# This test will only get the buy and sell prices of Mithril Ore
desired_item = materials["mithril"]
desired_id = desired_item["id"]
desired_fields = "name,buy,sell,supply,demand"

full_endpoint_path = base_url + endpoint_selected + "?ids=" + desired_id + "&fields=" + desired_fields
response = requests.get(full_endpoint_path)
current_prices = json.loads(response.content)

item_buy_price = current_prices['results'][0][2]
item_sell_price = current_prices['results'][0][3]
item_supply = current_prices['results'][0][4]
item_demand = current_prices['results'][0][5]

print("-------------------------------")
print(desired_item['name'])
print("Buy: ", item_buy_price, "|", "Sell: ", item_sell_price)
print("Demand: ", item_demand, "|", "Supply: ", item_supply)
print("-------------------------------")
