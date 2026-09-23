SRC_YAMLS := blog.secret.yaml blog.config.yaml blog.pod.yaml
BUNDLE := blog.bundle.yaml

.PHONY: all clean

all: $(BUNDLE)

$(BUNDLE): $(SRC_YAMLS)
	@rm -f $(BUNDLE)
	@for f in $(SRC_YAMLS); do \
		cat "$$f" >> $(BUNDLE); \
		echo "" >> $(BUNDLE); \
		echo "---" >> $(BUNDLE); \
		echo "" >> $(BUNDLE); \
	done
	@chmod 400 $(BUNDLE)
	@echo "Generated $(BUNDLE) successfully."

clean:
	@rm -f $(BUNDLE)
