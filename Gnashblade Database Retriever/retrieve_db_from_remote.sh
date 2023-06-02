echo "Starting Script - Copy Guild Wars 2 Trading Post data from server (Raspberry Pi) to local"

remote_host=raspberrypi # Connected with Tailscale, so no explicit IP is necessary
remote_username=gw2stuff
remote_file_location="/home/"$remote_username"/rust_tp_collector/material_listings.db"

# Copy the file
scp $remote_username@$remote_host:$remote_file_location ./

# Rename to include the current timestamp
time_of_file=$(date +%Y%m%d%H%M%S)
old_filename='material_listings' # Not including the .db extension
new_filename=$old_filename'_'$time_of_file'.db'

mv material_listings.db databases/material_listings_$time_of_file.db