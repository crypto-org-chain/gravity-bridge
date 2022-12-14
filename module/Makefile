PACKAGES=$(shell go list ./... | grep -v '/simulation')
VERSION := $(shell git describe --abbrev=6 --dirty --always --tags)
COMMIT := $(shell git log -1 --format='%H')
DOCKER := $(shell which docker)
DOCKER_BUF := $(DOCKER) run --rm -v $(CURDIR):/workspace --workdir /workspace bufbuild/buf
HTTPS_GIT := https://github.com/peggyjv/gravity-bridge.git
COVERAGE ?= coverage.txt
PROJECT_NAME = $(shell git remote get-url origin | xargs basename -s .git)

build_tags = netgo
ifeq ($(LEDGER_ENABLED),true)
  ifeq ($(OS),Windows_NT)
    GCCEXE = $(shell where gcc.exe 2> NUL)
    ifeq ($(GCCEXE),)
      $(error gcc.exe not installed for ledger support, please install or set LEDGER_ENABLED=false)
    else
      build_tags += ledger
    endif
  else
    UNAME_S = $(shell uname -s)
    ifeq ($(UNAME_S),OpenBSD)
      $(warning OpenBSD detected, disabling ledger support (https://github.com/cosmos/cosmos-sdk/issues/1988))
    else
      GCC = $(shell command -v gcc 2> /dev/null)
      ifeq ($(GCC),)
        $(error gcc not installed for ledger support, please install or set LEDGER_ENABLED=false)
      else
        build_tags += ledger
      endif
    endif
  endif
endif

ifeq (cleveldb,$(findstring cleveldb,$(GAIA_BUILD_OPTIONS)))
  build_tags += gcc
endif
build_tags += $(BUILD_TAGS)
build_tags := $(strip $(build_tags))

whitespace :=
whitespace += $(whitespace)
comma := ,
build_tags_comma_sep := $(subst $(whitespace),$(comma),$(build_tags))

ldflags = -X github.com/cosmos/cosmos-sdk/version.Name=gravity \
	-X github.com/cosmos/cosmos-sdk/version.AppName=gravity \
	-X github.com/cosmos/cosmos-sdk/version.Version=$(VERSION) \
	-X github.com/cosmos/cosmos-sdk/version.Commit=$(COMMIT) \
	-X "github.com/cosmos/cosmos-sdk/version.BuildTags=$(build_tags_comma_sep)" \

BUILD_FLAGS := -ldflags '$(ldflags)' -gcflags="all=-N -l"

all: install

install: go.sum
		go install $(BUILD_FLAGS) ./cmd/gravity

go.sum: go.mod
		@echo "--> Ensure dependencies have not been modified"
		GO111MODULE=on go mod verify

test:
	@go test -mod=readonly $(PACKAGES)

test-cov:
	@go test -mod=readonly $(PACKAGES) -coverprofile=$(COVERAGE) -covermode=atomic

build:
	go build -o build/gravity $(BUILD_FLAGS) ./cmd/gravity/main.go

docker-build:
	docker build -t github.com/althea-net/gravity:latest

# look into .golangci.yml for enabling / disabling linters
lint:
	@echo "--> Running linter"
	@golangci-lint run
	@go mod verify

###############################################################################
###                           Protobuf                                    ###
###############################################################################
# ------
# NOTE: Link to the tendermintdev/sdk-proto-gen docker images:
#       https://hub.docker.com/r/tendermintdev/sdk-proto-gen/tags
#
protoVer=v0.7
protoImageName=tendermintdev/sdk-proto-gen:$(protoVer)
protoImage=$(DOCKER) run --network host --rm -v $(CURDIR):/workspace --workdir /workspace $(protoImageName)
# ------
# NOTE: cosmos/proto-builder image is needed because clang-format is not installed
#       on the tendermintdev/sdk-proto-gen docker image.
#		Link to the cosmos/proto-builder docker images:
#       https://github.com/cosmos/cosmos-sdk/pkgs/container/proto-builder
#
protoCosmosVer=0.11.2
protoCosmosName=ghcr.io/cosmos/proto-builder:$(protoCosmosVer)
protoCosmosImage=$(DOCKER) run --network host --rm -v $(CURDIR):/workspace --workdir /workspace $(protoCosmosName)
# ------
# NOTE: Link to the yoheimuta/protolint docker images:
#       https://hub.docker.com/r/yoheimuta/protolint/tags
#
protolintVer=0.42.2
protolintName=yoheimuta/protolint:$(protolintVer)
protolintImage=$(DOCKER) run --network host --rm -v $(CURDIR):/workspace --workdir /workspace $(protolintName)

proto-all: proto-format proto-lint proto-gen

proto-format:
	@echo "Formatting Protobuf files"
	$(DOCKER) run --rm -v $(CURDIR):/workspace \
	--workdir /workspace tendermintdev/docker-build-proto \
	find ./ -not -path "./third_party/*" -name *.proto -exec clang-format -i {} \;

proto-gen:
	@echo "Generating Protobuf files"
	$(protoCosmosImage) sh ./contrib/local/protocgen.sh

# TODO: Rethink API docs generation
# proto-swagger-gen:
# 	@echo "Generating Protobuf Swagger"
# 	$(protoImage) sh ./scripts/protoc-swagger-gen.sh

proto-format:
	@echo "Formatting Protobuf files"
	$(protoCosmosImage) find ./ -name *.proto -exec clang-format -i {} \;

# NOTE: The linter configuration lives in .protolint.yaml
proto-lint:
	@echo "Linting Protobuf files"
	$(protolintImage) lint ./proto

proto-check-breaking:
	@echo "Checking Protobuf files for breaking changes"
	$(protoCosmosImage) buf breaking --against $(HTTPS_GIT)#branch=main

TM_URL           = https://raw.githubusercontent.com/tendermint/tendermint/v0.34.0-rc3/proto/tendermint
GOGO_PROTO_URL   = https://raw.githubusercontent.com/regen-network/protobuf/cosmos
COSMOS_PROTO_URL = https://raw.githubusercontent.com/regen-network/cosmos-proto/master
COSMOS_SDK_PROTO_URL = https://raw.githubusercontent.com/cosmos/cosmos-sdk/master/proto/cosmos/base
GOOGLE_PROTO_URL   = https://raw.githubusercontent.com/googleapis/googleapis/master/google/api
PROTOBUF_GOOGLE_URL = https://raw.githubusercontent.com/protocolbuffers/protobuf/master/src/google/protobuf

TM_CRYPTO_TYPES     = third_party/proto/tendermint/crypto
TM_ABCI_TYPES       = third_party/proto/tendermint/abci
TM_TYPES     	    = third_party/proto/tendermint/types
TM_VERSION 			= third_party/proto/tendermint/version
TM_LIBS				= third_party/proto/tendermint/libs/bits

GOGO_PROTO_TYPES    = third_party/proto/gogoproto
COSMOS_PROTO_TYPES  = third_party/proto/cosmos_proto
GOOGLE_PROTO_TYPES  = third_party/proto/google/api
PROTOBUF_GOOGLE_TYPES = third_party/proto/google/protobuf

SDK_ABCI_TYPES  	= third_party/proto/cosmos/base/abci/v1beta1
SDK_QUERY_TYPES  	= third_party/proto/cosmos/base/query/v1beta1
SDK_COIN_TYPES  	= third_party/proto/cosmos/base/v1beta1

proto-update-deps:
	mkdir -p $(GOGO_PROTO_TYPES)
	curl -sSL $(GOGO_PROTO_URL)/gogoproto/gogo.proto > $(GOGO_PROTO_TYPES)/gogo.proto

	mkdir -p $(COSMOS_PROTO_TYPES)
	curl -sSL $(COSMOS_PROTO_URL)/cosmos.proto > $(COSMOS_PROTO_TYPES)/cosmos.proto

	mkdir -p $(TM_ABCI_TYPES)
	curl -sSL $(TM_URL)/abci/types.proto > $(TM_ABCI_TYPES)/types.proto

	mkdir -p $(TM_VERSION)
	curl -sSL $(TM_URL)/version/types.proto > $(TM_VERSION)/types.proto

	mkdir -p $(TM_TYPES)
	curl -sSL $(TM_URL)/types/types.proto > $(TM_TYPES)/types.proto
	curl -sSL $(TM_URL)/types/evidence.proto > $(TM_TYPES)/evidence.proto
	curl -sSL $(TM_URL)/types/params.proto > $(TM_TYPES)/params.proto

	mkdir -p $(TM_CRYPTO_TYPES)
	curl -sSL $(TM_URL)/crypto/proof.proto > $(TM_CRYPTO_TYPES)/proof.proto
	curl -sSL $(TM_URL)/crypto/keys.proto > $(TM_CRYPTO_TYPES)/keys.proto

	mkdir -p $(TM_LIBS)
	curl -sSL $(TM_URL)/libs/bits/types.proto > $(TM_LIBS)/types.proto

	mkdir -p $(SDK_ABCI_TYPES)
	curl -sSL $(COSMOS_SDK_PROTO_URL)/abci/v1beta1/abci.proto > $(SDK_ABCI_TYPES)/abci.proto

	mkdir -p $(SDK_QUERY_TYPES)
	curl -sSL $(COSMOS_SDK_PROTO_URL)/query/v1beta1/pagination.proto > $(SDK_QUERY_TYPES)/pagination.proto

	mkdir -p $(SDK_COIN_TYPES)
	curl -sSL $(COSMOS_SDK_PROTO_URL)/v1beta1/coin.proto > $(SDK_COIN_TYPES)/coin.proto

	mkdir -p $(GOOGLE_PROTO_TYPES)
	curl -sSL $(GOOGLE_PROTO_URL)/annotations.proto > $(GOOGLE_PROTO_TYPES)/annotations.proto
	curl -sSL $(GOOGLE_PROTO_URL)/http.proto > $(GOOGLE_PROTO_TYPES)/http.proto
	curl -sSL $(GOOGLE_PROTO_URL)/httpbody.proto > $(GOOGLE_PROTO_TYPES)/httpbody.proto

	mkdir -p $(PROTOBUF_GOOGLE_TYPES)
	curl -sSL $(PROTOBUF_GOOGLE_URL)/any.proto > $(PROTOBUF_GOOGLE_TYPES)/any.proto

###############################################################################
###                                Sim Test                                 ###
###############################################################################

SIMAPP = github.com/peggyjv/gravity-bridge/module/v2/app

test-sim-nondeterminism:
	@echo "Running non-determinism test..."
	@go test -mod=readonly $(SIMAPP) -run TestAppStateDeterminism -Enabled=true \
		-NumBlocks=100 -BlockSize=200 -Commit=true -Period=0 -v -timeout 24h