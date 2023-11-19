#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex
./init.coffee

source ../../../../../../conf/env/srv_host.sh

./hostNew.coffee $SRV_HOST
../cron/banTld.coffee
