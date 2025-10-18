FROM python:3.14
COPY generate/requirements.txt requirements.txt
RUN --mount=type=cache,target=/root/.cache/pip python -m pip install -r requirements.txt
