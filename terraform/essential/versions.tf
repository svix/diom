terraform {
  required_version = "~> 1.0"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 6.34"
    }

    kubernetes = {
      version = "~> 3.0"
    }

    kubectl = {
      source  = "alekc/kubectl"
      version = "~> 2.1"
    }

    helm = {
      version = "~> 3.1"
    }

    tfe = {
      version = "~> 0.73"
    }

    tls = {
      version = "~> 4.2"
    }

    local = {
      version = "~> 2.6"
    }

    random = {
      version = "~> 3.8"
    }

    # datadog = {
    #   source  = "datadog/datadog"
    #   version = "~> 3.86"
    # }
  }

  backend "remote" {
    organization = "svix-development"
    # organization = ""
    # organization = ""
    workspaces {
      prefix = "coyote-essential-"
    }
  }
}
