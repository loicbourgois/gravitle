FROM python:3.10
COPY ai/requirements.txt requirements.txt
RUN --mount=type=cache,target=/root/.cache/pip python -m pip install -r requirements.txt
