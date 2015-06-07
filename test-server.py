import json
from bottle import route, run, request, response

@route('/', method='POST')
def index():
	print request.json
	return request.json["hello"]
	

run(host='localhost', port=8080)
