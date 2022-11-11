import requests

from constants import SESSION_BACKEND_HOST, HTTP_200_OK, HTTP_400_BAD_REQUEST, SERVER_INTERNAL_ERROR


def get_session_timetable(group) -> [str, str]:
    try:
        response = requests.get(
            f'{SESSION_BACKEND_HOST}/session/{group}',
            timeout=10
        )
    except ConnectionError:
        return None, SERVER_INTERNAL_ERROR

    if response.status_code == HTTP_200_OK:
        return response.json(), None
    if response.status_code == HTTP_400_BAD_REQUEST:
        return None, response.json()['message']
