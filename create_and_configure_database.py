import sqlite3
from sqlite3 import Error
import json

def create_connection(db_file):
    conn = None
    try:
        conn = sqlite3.connect(db_file)
        return conn
    except Error as e:
        print(e)
    return conn

def create_table(conn,create_table_SQL):
    try:
        c = conn.cursor()
        c.execute(create_table_SQL)
    except Error as e:
        print(e)

db_file = r"./material_listings.db"

sql_create_items_table = """CREATE TABLE IF NOT EXISTS items (
item_id integer PRIMARY KEY,
item_name TEXT NOT NULL, 
material_type TEXT,
material_tier INTEGER
);"""

sql_create_listings_table = """ CREATE TABLE IF NOT EXISTS listings (
request_time TEXT,
request_timestamp TEXT,
item_id INTEGER NOT NULL,
buy_price INTEGER NOT NULL,
buy_number_of_listings INTEGER NOT NULL, 
buy_quantity INTEGER NOT NULL,
sell_price INTEGER NOT NULL,
sell_number_of_listings INTEGER NOT NULL,
sell_quantity INTEGER NOT NULL,
full_buy_list BLOB,
full_sell_list BLOB,
FOREIGN KEY (item_id) REFERENCES items (item_id)
);"""

f = open('materials.json')
materials = json.load(f)
f.close()

conn = create_connection(db_file)
if conn is not None:
    create_table(conn, sql_create_items_table)
    create_table(conn, sql_create_listings_table)
else:
    print("Error! cannot create the database connection.")

for item in materials.items():
    item_to_insert = (item[1]['id'],item[1]['name'],item[1]['type'],item[1]['tier'])
    sql_insert_item = ''' INSERT INTO items(item_id,item_name,material_type,material_tier)
                 VALUES(?,?,?,?) '''
    cur = conn.cursor()
    cur.execute(sql_insert_item, item_to_insert)
    conn.commit()
