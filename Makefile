.PHONY: clean
clean:

.PHONY: build
build:
	bin/build

.PHONY: test
test:
	bin/test

.PHONY: vet
vet:
	bin/vet

.PHONY: update
update:
	bin/update
