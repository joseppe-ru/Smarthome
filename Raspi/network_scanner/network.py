import sys

# eingene Import
from network.arp_scanner import Network_Scanner

sys.path.append("ip_scanner")

scanner = Network_Scanner()
scanner.startup()
ip_adr_test = "69.42.0.101"

if (scanner.ping_check(ip_adr_test)):
    print("IP gefunden")
else:
    print("IP nicht gefunden")