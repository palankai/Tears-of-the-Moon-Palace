ifneq (,$(wildcard .env))
	include .env
	export $(shell sed 's/=.*//' .env)
endif

.PHONY: all build dist

all: help					# Default command

help:						# This help
	@echo "Makefile commands:"
	@echo
	@grep '^[^#\.[:space:]].*:[^=]*$$' Makefile

dist: prototype-dist				# dist everything
	rm -rf dist
	mkdir -p dist
	cp -r prototype/dist/*.zip dist

prototype-dist:					# Dist the prototype
	cd prototype && make dist
