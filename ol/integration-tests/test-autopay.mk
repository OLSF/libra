SHELL=/usr/bin/env bash
DATA_PATH = ${HOME}/.0L
SWARM_TEMP = ${DATA_PATH}/swarm_temp
LOG=${DATA_PATH}/test-autopay.log
UNAME := $(shell uname)

NODE_ENV=test
TEST=y

ifndef SOURCE_PATH
SOURCE_PATH = ${HOME}/libra
endif
MAKE_FILE = ${SOURCE_PATH}/ol/integration-tests/test-autopay.mk

# alice
ifndef PERSONA
PERSONA=alice
endif

MNEM="talent sunset lizard pill fame nuclear spy noodle basket okay critic grow sleep legend hurry pitch blanket clerk impose rough degree sock insane purse"

NUM_NODES = 2

START_TEXT = "To run the Libra CLI client"

ifndef SUCCESS_TEXT
SUCCESS_TEXT = "transaction executed"
endif

ifndef AUTOPAY_FILE
AUTOPAY_FILE = alice.autopay_batch.json
endif

export

test: swarm check-swarm set-community send-tx check-tx  check-autopay check-transfer stop

test-percent-bal:
	AUTOPAY_FILE=alice.autopay_batch.json make -f ${MAKE_FILE} test

test-fixed-once:
	AUTOPAY_FILE=alice.fixed_once.autopay_batch.json make -f ${MAKE_FILE} test

test-all:
	export AUTOPAY_FILE=all.autopay_batch.json SUCCESS_TEXT="'with sequence number: 7'" && make -f ${MAKE_FILE} test

swarm:
	@echo Building Swarm
	rm -rf ${SWARM_TEMP}
	mkdir ${SWARM_TEMP}
	cd ${SOURCE_PATH} && cargo build -p libra-node -p cli
	cd ${SOURCE_PATH} && cargo run -p libra-swarm -- --libra-node ${SOURCE_PATH}/target/debug/libra-node -c ${SWARM_TEMP} -n ${NUM_NODES} &> ${LOG} &

stop:
	killall libra-swarm libra-node miner ol txs cli | true

init:
	cd ${SOURCE_PATH} && cargo r -p ol -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} init --source-path ${SOURCE_PATH}

tx:
	cd ${SOURCE_PATH} && NODE_ENV=test TEST=y cargo r -p txs -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} autopay-batch -f ${SOURCE_PATH}/ol/fixtures/autopay/${AUTOPAY_FILE}

set-community:
	cd ${SOURCE_PATH} && NODE_ENV=test TEST=y cargo r -p txs -- --swarm-path ${SWARM_TEMP} --swarm-persona bob wallet -c

resources:
	cd ${SOURCE_PATH} && cargo run -p ol -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} query --resources

balance:
	cd ${SOURCE_PATH} && cargo run -p ol -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} query --balance

balance-bob:
	cd ${SOURCE_PATH} && cargo run -p ol -- --account 88E74DFED34420F2AD8032148280A84B --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} query --balance


check-swarm: 
	@while [[ ${NOW} -le ${END} ]] ; do \
			if grep -q ${START_TEXT} ${LOG} ; then \
				break; \
			else \
				echo . ; \
			fi ; \
			echo "Sleeping for 5 secs" ; \
			sleep 5 ; \
	done

send-tx: 
	PERSONA=alice make -f ${MAKE_FILE} init
	PERSONA=alice make -f ${MAKE_FILE} tx &>> ${LOG} &

check-tx:
	@while [[ ${NOW} -le ${END} ]] ; do \
			if grep -q ${SUCCESS_TEXT} ${LOG} ; then \
				echo TX SUCCESS ; \
				break ; \
			else \
				echo . ; \
			fi ; \
			echo "Sleeping for 5 secs" ; \
			sleep 5 ; \
	done

check-autopay: 
# checks if there is any mention of BOB's account as a payee
	PERSONA=alice make -f ${MAKE_FILE} resources | grep -e '88E74DFED34420F2AD8032148280A84B' -e 'payee'


check-transfer:
# swarm accounts start with a balance of 4
	@while [[ ${NOW} -le ${END} ]] ; do \
			if PERSONA=alice make -f ${MAKE_FILE} balance-bob | grep -e '5'; then \
				echo TX SUCCESS ; \
				break ; \
			else \
				echo . ; \
			fi ; \
			echo "Sleeping for 5 secs" ; \
			sleep 5 ; \
	done
	