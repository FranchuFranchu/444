from http.server import SimpleHTTPRequestHandler
import http
from socketserver import TCPServer


class NoCacheHTTPRequestHandler(
    http.server.SimpleHTTPRequestHandler
):
    def send_response_only(self, code, message=None):
        super().send_response_only(code, message)
        self.send_header('Cache-Control', 'no-store, must-revalidate')
        self.send_header('Expires', '0')

def run():
    Handler = NoCacheHTTPRequestHandler
    Handler.extensions_map = {'': 'text/plain', 'html': 'text/html', 'js': 'application/javascript'}
    with TCPServer(('', 8001), Handler) as httpd:
        httpd.serve_forever()


if __name__ == '__main__':
    run()