from requests import Session, Response


class TheBlueApi:
    def __init__(self, api_key: str, host='https://www.thebluealliance.com/api/v3'):
        self.api_key = api_key
        self.host = host

        self._session = Session()
        self._session.headers['X-TBA-Auth-Key'] = api_key

    def get(self, url: str) -> Response:
        response = self._session.get(self.host + url)
        response.raise_for_status()
        return response
