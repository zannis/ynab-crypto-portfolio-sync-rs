FROM ubuntu:latest
LABEL authors="zannis"

ENTRYPOINT ["top", "-b"]