# Deployment instance name. Override it to deploy a second instance, e.g.
#   make NAME=blog-beta
# The __NAME__ placeholder covers the Pod name, the Secret names, the volume
# names and the PersistentVolumeClaim references; the `image:` fields are
# handled by the *_IMAGE / *_TAG variables below instead.
NAME ?= blog

# Host ports each instance binds on 127.0.0.1. Give a second instance its own
# ports so it can share a node with the primary one, e.g.
#   make NAME=blog-beta FRONTEND_PORT=10021 BACKEND_PORT=10022
FRONTEND_PORT ?= 10011
BACKEND_PORT ?= 10012

# Container images, given as repository + tag. The repository is the same for
# every instance, so normally only the tag is overridden, e.g. to run a beta
# instance off `latest`:
#   make NAME=blog-beta BACKEND_TAG=latest FRONTEND_TAG=latest
BACKEND_IMAGE  ?= registry.squidspirit.com/squid/blog-backend
BACKEND_TAG    ?= v0.8.0
FRONTEND_IMAGE ?= registry.squidspirit.com/squid/blog-frontend
FRONTEND_TAG   ?= v0.8.0

# Source files are picked per instance: <NAME>.secret.yaml, <NAME>.config.yaml
# and <NAME>.pod.yaml are used when they exist, otherwise the shared blog.*
# file is used. Either way the __NAME__ / port placeholders are substituted, so
# a new instance only needs its own file when some values actually differ.
SECRET ?= $(firstword $(wildcard $(NAME).secret.yaml) blog.secret.yaml)
CONFIG ?= $(firstword $(wildcard $(NAME).config.yaml) blog.config.yaml)
POD    ?= $(firstword $(wildcard $(NAME).pod.yaml) blog.pod.yaml)

SRC_YAMLS := $(SECRET) $(CONFIG) $(POD)
BUNDLE := $(NAME).bundle.yaml

.PHONY: all clean

all: $(BUNDLE)

$(BUNDLE): $(SRC_YAMLS)
	@rm -f $(BUNDLE)
	@echo "Bundling $(SRC_YAMLS)"
	@touch $(BUNDLE)
	@chmod 600 $(BUNDLE)
	@for f in $(SRC_YAMLS); do \
		sed -e "s|__NAME__|$(NAME)|g" \
		    -e "s|__FRONTEND_PORT__|$(FRONTEND_PORT)|g" \
		    -e "s|__BACKEND_PORT__|$(BACKEND_PORT)|g" \
		    -e "s|__FRONTEND_IMAGE__|$(FRONTEND_IMAGE)|g" \
		    -e "s|__BACKEND_IMAGE__|$(BACKEND_IMAGE)|g" \
		    -e "s|__FRONTEND_TAG__|$(FRONTEND_TAG)|g" \
		    -e "s|__BACKEND_TAG__|$(BACKEND_TAG)|g" "$$f" >> $(BUNDLE); \
		echo "" >> $(BUNDLE); \
		echo "---" >> $(BUNDLE); \
		echo "" >> $(BUNDLE); \
	done
	@chmod 400 $(BUNDLE)
	@echo "Generated $(BUNDLE) successfully."

clean:
	@rm -f $(BUNDLE)
