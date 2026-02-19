FROM docker.io/golang:1.24-alpine
RUN go install golang.org/x/tools/cmd/goimports@latest
