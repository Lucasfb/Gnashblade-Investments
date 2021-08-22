if __name__ == '__main__':
    import requests
    import json

    # Loading the data of items of interest from a file
    # Data is manually added to the file
    # Could change to a database as it gets bigger or needs better selection methods
    f = open('materials.json')
    materials = json.load(f)
    f.close()

    base_url = "https://api.guildwars2.com/v2/"
    endpoint_selected = "commerce/prices"

    # This test will only get the buy and sell prices of Mithril Ore
    desired_item = materials["mithril"]
    desired_id = desired_item["id"]

    full_endpoint_path = base_url + endpoint_selected + "?ids=" + desired_id
    response = requests.get(full_endpoint_path)
    current_prices = json.loads(response.content)
    # print(response.json())

    # Showing the results in a pretty way
    item_buy_price = current_prices[0]['buys']['unit_price']
    item_sell_price = current_prices[0]['sells']['unit_price']

    print("-------------------------------")
    print(desired_item['name'])
    print("Buy: ", item_buy_price , "|", "Sell: ", item_sell_price)
    print("-------------------------------")
