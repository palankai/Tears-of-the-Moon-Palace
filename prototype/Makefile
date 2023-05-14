ifneq (,$(wildcard .env))
	include .env
	export $(shell sed 's/=.*//' .env)
endif

NAME := tears_of_the_moon_palace-prototype

.PHONY: all build dist release

all: help								# Default command

help:									# This help
	@echo "Makefile commands:"
	@echo
	@grep '^[^#\.[:space:]].*:[^=]*$$' Makefile

build:									# Build the prototype
	cargo build

release: release-x86_64-apple-darwin release-aarch64-apple-darwin	# Release everything


dist: dist-clean dist-x86_64-apple-darwin dist-aarch64-apple-darwin	# Dist everything

dist-clean:								# Clean dist
	rm -rf dist
	mkdir -p dist


release-x86_64-apple-darwin:						# Release the prototype for x86_64-apple-darwin
	cargo build --release --target x86_64-apple-darwin

dist-x86_64-apple-darwin: release-x86_64-apple-darwin 			# Dist the prototype for x86_64-apple-darwin
	rm -rf dist/x86_64-apple-darwin
	mkdir -p dist/x86_64-apple-darwin/$(NAME)-apple-x86_64
	cp -r assets/ dist/x86_64-apple-darwin/$(NAME)-apple-x86_64
	cp target/x86_64-apple-darwin/release/$(NAME) dist/x86_64-apple-darwin/$(NAME)-apple-x86_64/$(NAME)-apple-x86_64
	cd dist/x86_64-apple-darwin && zip -r ../$(NAME)-apple-x86_64.zip $(NAME)-apple-x86_64


release-aarch64-apple-darwin:						# Release the prototype for aarch64-apple-darwin
	cargo build --release --target aarch64-apple-darwin

dist-aarch64-apple-darwin: release-aarch64-apple-darwin 		# Dist the prototype for aarch64-apple-darwin
	rm -rf dist/aarch64-apple-darwin
	mkdir -p dist/aarch64-apple-darwin/$(NAME)-apple-arm64
	cp -r assets/ dist/aarch64-apple-darwin/$(NAME)-apple-arm64
	cp target/aarch64-apple-darwin/release/$(NAME) dist/aarch64-apple-darwin/$(NAME)-apple-arm64/$(NAME)-apple-arm64
	cd dist/aarch64-apple-darwin && zip -r ../$(NAME)-apple-arm64.zip $(NAME)-apple-arm64
