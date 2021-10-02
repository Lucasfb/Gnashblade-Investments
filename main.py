if __name__ == '__main__':
    import requests
    import json
    import datetime
    import matplotlib.pyplot as plt
    import numpy as np

    # Loading the data of items of interest from a file
    # Data is manually added to the file
    # Could change to a database as it gets bigger or needs better selection methods
    f = open('materials.json')
    materials = json.load(f)
    f.close()

    # Selecting the "listings" endpoint
    base_url = "https://api.guildwars2.com/v2"
    endpoint_selected = "commerce/listings"

    # This test will only get the buy and sell prices of Mithril Ore
    desired_item = materials["mithril"]
    desired_id = desired_item["id"]

    full_endpoint_path = base_url + "/" + endpoint_selected + "?ids=" + desired_id

    response = requests.get(full_endpoint_path)
    # Considers the difference between requesting and getting the current time negligible
    request_time = datetime.datetime.now()
    request_timestamp = request_time.timestamp()
    current_listings = json.loads(response.content)
    current_listings = current_listings[0]
    current_listings['timestamp'] = request_timestamp
    current_listings['request_time'] = request_time
    # print(response.json())

    # Showing the results in a pretty way
    item_buy_price = current_listings['buys'][0]['unit_price']
    item_sell_price = current_listings['sells'][0]['unit_price']

    print("-------------------------------")
    print(desired_item['name'])
    print("  Highest Buy Price: ", item_buy_price)
    print("  Lowest Sell Price: ", item_sell_price)
    print("-------------------------------")

    # # Bar graph of current listings
    # ## Bar graph of buy listings
    # buy_values = []
    # buy_quantities = []
    # for listing in current_listings['buys']:
    #     buy_values.append(listing['unit_price'])
    #     buy_quantities.append(listing['quantity'])
    # fif, ax = plt.subplots()
    # ax.barh(buy_values, buy_quantities, align='center')
    # ax.set_yticks(np.arange(0, max(buy_values)+2, 1))
    # #ax.set_xlim(right=15)  # adjust xlim to fit labels
    # plt.ylabel("Buy price (copper)")
    # plt.xlabel("Quantity listed")
    # plt.title("Number of listings for all buy price")
    # plt.show()
