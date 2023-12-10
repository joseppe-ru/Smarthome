import socket
import scapy.all as scapy

class Network_Scanner:

    def startup(self):
        print("Localhost: "+socket.gethostbyname(socket.gethostname()))
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.connect(("8.8.8.8", 80))
        print("(eth0) ip-addr: "+s.getsockname()[0])

    def scan(self):
        scapy.arping("10.42.0.101")