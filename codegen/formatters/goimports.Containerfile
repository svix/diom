FROM docker.io/golang:1.26-alpine
RUN go install golang.org/x/tools/cmd/goimports@latest
