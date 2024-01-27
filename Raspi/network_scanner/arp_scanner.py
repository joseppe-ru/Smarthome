import socket
import subprocess

class Network_Scanner:

    def startup(self):
        print("Localhost: "+socket.gethostbyname(socket.gethostname()))
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.connect(("8.8.8.8", 80))
        print("(eth0) ip-addr: "+s.getsockname()[0])

    def ping_check(self, ip_adr):
        try:
            subprocess.check_output(["ping", ip_adr])
            return True
        except subprocess.CalledProcessError:
            return False
        