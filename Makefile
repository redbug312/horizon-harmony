METRO_USERNAME ?= ???
METRO_PASSWORD ?= ???
TDX_ID ?= ???
TDX_SECRET ?= ???

SECRETS ?= METRO_USERNAME=$(METRO_USERNAME) \
	METRO_PASSWORD=$(METRO_PASSWORD) \
	TDX_ID=$(TDX_ID) \
	TDX_SECRET=$(TDX_SECRET)

.PHONY: run
run:
	rinf message
	$(SECRETS) flutter run

.PHONY: test-api
test-api:
	TDX_ACCESS_TOKEN=$(shell curl \
		--request POST \
		--url 'https://tdx.transportdata.tw/auth/realms/TDXConnect/protocol/openid-connect/token' \
		--header 'content-type: application/x-www-form-urlencoded' \
		--data grant_type=client_credentials \
		--data client_id=$(TDX_ID) \
		--data client_secret=$(TDX_SECRET) | jq .access_token) \
	$(SECRETS) cargo test -- --include-ignored

.PHONY: test-for-ci
test-for-ci: test-api
