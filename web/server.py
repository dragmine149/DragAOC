# Python 3 server example
from http.server import BaseHTTPRequestHandler, HTTPServer
import time
import requests
import datetime
from random import randint

hostName = "localhost"
serverPort = 9090

class MyServer(BaseHTTPRequestHandler):
	def do_GET(self):
		self.send_response(200)
		self.send_header("Content-type", "text/html")
		self.end_headers()

		if self.path == '/':
			dt = datetime.date.today();
			year = dt.year
			if dt.month < 12:
				year -= 1

			self.path = '/' + str(year)

		if self.path != '/':
			cookies = {
 				'session': '53616c7465645f5fac8f5b7ad5506f9c1d0fd78a3d73a64b9a2596d5d668718cc211fd9cbc541bbc64b3bef6f4fbbf6d225504de8c99f21c31daca57b25a2da6',
			}
			headers = {
				'User-Agent': 'Python local server test by dan@thebanners.uk'
			}
			r = requests.get('https://adventofcode.com' + self.path, cookies=cookies, headers=headers)
#			print(r.text)
			self.wfile.write(bytes(r.text, 'utf-8'))

#        self.wfile.write(bytes("<html><head><title>https://pythonbasics.org</title></head>", "utf-8"))
#        self.wfile.write(bytes("<p>Request: %s</p>" % self.path, "utf-8"))
#        self.wfile.write(bytes("<body>", "utf-8"))
#        self.wfile.write(bytes("<p>This is an example web server.</p>", "utf-8"))
#        self.wfile.write(bytes("</body></html>", "utf-8"))

if __name__ == "__main__":
	webServer = HTTPServer((hostName, serverPort), MyServer)
	print("Server started http://%s:%s" % (hostName, serverPort))

	try:
		webServer.serve_forever()
	except KeyboardInterrupt:
		pass

	webServer.server_close()
	print("Server stopped.")
