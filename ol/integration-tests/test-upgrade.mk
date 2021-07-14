SHELL=/usr/bin/env bash
DATA_PATH = ${HOME}/.0L
SWARM_TEMP = ${DATA_PATH}/swarm_temp
UPGRADE_TEMP = ${DATA_PATH}/test-upgrade
SAFE_MAKE_FILE = ${UPGRADE_TEMP}/test-upgrade.mk
LOG=${UPGRADE_TEMP}/test-upgrade.log
UNAME := $(shell uname)

NODE_ENV=test
TEST=y

ifndef SOURCE_PATH
SOURCE_PATH = ${HOME}/libra
endif

# alice
ifndef PERSONA
PERSONA=alice
endif

MNEM="talent sunset lizard pill fame nuclear spy noodle basket okay critic grow sleep legend hurry pitch blanket clerk impose rough degree sock insane purse"

NUM_NODES = 2

ifndef PREV_VERSION
PREV_VERSION=v4.3.1
endif

ifndef BRANCH_NAME
BRANCH_NAME=main
endif

# USAGE: BRANCH_NAME=<latest branch> make -f test-upgrade.mk upgrade-test
# NOTE: BRANCH_NAME shares semantics with https://github.com/marketplace/actions/get-branch-name
test: prep get-prev stdlib start upgrade check progress stop

start:
	@echo Building Swarm
	cd ${SOURCE_PATH} && cargo build -p libra-node -p cli
	cd ${SOURCE_PATH} && cargo run -p libra-swarm -- --libra-node ${SOURCE_PATH}/target/debug/libra-node -c ${SWARM_TEMP} -n ${NUM_NODES} &> ${LOG}&

stop:
	killall libra-swarm libra-node miner ol txs cli | true

prep:
# save makefile outside of repo, since we'll need it across branches
#	mkdir ${HOME}/.0L/ | true
	mkdir -p ${UPGRADE_TEMP} | true
	cp ${SOURCE_PATH}/ol/integration-tests/test-upgrade.mk ${SAFE_MAKE_FILE}

get-prev:
	cd ${SOURCE_PATH} && git reset --hard && git fetch
	cd ${SOURCE_PATH} && git checkout ${PREV_VERSION} -f

get-test:
	cd ${SOURCE_PATH} && git reset --hard && git fetch
	cd ${SOURCE_PATH} && git checkout ${BRANCH_NAME} -f

stdlib:
	cd ${SOURCE_PATH} && cargo run --release -p stdlib
	cd ${SOURCE_PATH} && cargo run --release -p stdlib -- --create-upgrade-payload
	sha256sum ${SOURCE_PATH}/language/stdlib/staged/stdlib.mv

init:
	cd ${SOURCE_PATH} && cargo run -p ol -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} init
	cp ${SWARM_TEMP}/0/0L.toml ${HOME}/.0L/0L.toml

submit:
	cd ${SOURCE_PATH} && cargo run -p txs -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} oracle-upgrade -f ${SOURCE_PATH}/language/stdlib/staged/stdlib.mv

query:
	cd ${SOURCE_PATH} && cargo run -p ol -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} query --blockheight | grep -Eo [0-9]+ | tail -n1

txs:
	cd ${SOURCE_PATH} && cargo run -p txs -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} demo


ifeq ($(UNAME), Darwin)
END = $(shell date -v +5M +%s)
NOW = $(shell date -u +%s)
else 
END = $(shell date -ud "5 minutes" +%s)
NOW = $(shell date -u +%s)
endif

START_TEXT = "To run the Libra CLI client"
UPGRADE_TEXT = "stdlib upgrade: published"

upgrade: 
	@while [[ ${NOW} -le ${END} ]] ; do \
			if grep -q ${START_TEXT} ${LOG} ; then \
				make -f ${SAFE_MAKE_FILE} get-test stdlib ; \
				PERSONA=alice make -f ${SAFE_MAKE_FILE} submit; \
				PERSONA=bob make -f ${SAFE_MAKE_FILE} submit; \
				break; \
			else \
				echo . ; \
			fi ; \
			echo "Sleeping for 5 secs" ; \
			sleep 5 ; \
	done

check:	
	@while [[ ${NOW} -le ${END} ]] ; do \
			if grep -q ${UPGRADE_TEXT} ${SWARM_TEMP}/logs/0.log ; then \
				echo UPGRADE SUCCESS! ; \
				break ; \
			else \
				echo . ; \
			fi ; \
			echo "Sleeping for 5 secs" ; \
			sleep 5 ; \
	done

# check the blocks are progressing after upgrade
progress:
	@i=1 ; \
	while [[ $$i -le 10 ]] ; do \
			echo ===== Transaction $$i =====; \
			if make -f ${SAFE_MAKE_FILE} txs ; then \
				echo Making progress ; \
				i=$$(($$i + 1)); \
			else \
				echo ERROR, txs not successful ; \
				exit 1 ; \
			fi ; \
			echo "Sleeping for 1 min" ; \
			sleep 1m ; \
	done

tail:
	tail -f ${SWARM_TEMP}/logs/0/log