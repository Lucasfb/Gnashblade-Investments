from datetime import datetime
import sqlite3


# Connecting to the local database
conn = sqlite3.connect('material_listings.db')
cur = conn.cursor()

rows = cur.execute("SELECT request_time,request_timestamp FROM listings").fetchall()

for each_time in rows:
    (old_request_time,old_request_timestamp) = each_time
    new_request_time = datetime.fromisoformat(old_request_time).astimezone().isoformat()
    new_request_timestamp = int(float(old_request_timestamp)) # Convert from string to float, then to int
    cur.execute("UPDATE listings SET request_time = ?, request_timestamp = ? WHERE request_time= ?", [new_request_time, new_request_timestamp, old_request_time])
    conn.commit()

conn.close()