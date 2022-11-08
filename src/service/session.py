import requests

from constants import SESSION_BACKEND_HOST, HTTP_200_OK


def get_session_timetable(group) -> str:
    response = requests.get(
        f'{SESSION_BACKEND_HOST}/session/{group}'

    )
    if response.status_code == HTTP_200_OK:
        return response.json()

