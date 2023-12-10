# python main script, zum Steuern der Hausautmatisierungen, die auf dem Raspi laufen
# kommplett mnutzloser shit!!
import sys

# eingene Import
from network.arp_scanner import Network_Scanner

sys.path.append("ip_scanner")

scanner = Network_Scanner()
scanner.startup()
scanner.scan()