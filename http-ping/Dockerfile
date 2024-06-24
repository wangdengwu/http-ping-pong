FROM golang:alpine as builder
WORKDIR /usr/src/http-ping
COPY go.mod go.mod
COPY main.go main.go
RUN go build -o ping .

FROM alpine:latest
WORKDIR /usr/src/http-ping
COPY --from=builder /usr/src/http-ping/ping ./
EXPOSE 9999
ENTRYPOINT ./ping
