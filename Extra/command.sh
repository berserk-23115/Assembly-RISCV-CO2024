#!/bin/bash
CURR_DIREC="$(pwd)"
TEST_DIREC="TEST"
if [ ! -d "$TEST_DIREC" ]; then
  echo "TEST DIRECTORY NOT FOUND !"
  echo "CREATED , CREATE INPUT FILES IN THE TEST FOLDER"
  mkdir "$TEST_DIREC"
  mkdir "$TEST_DIREC/INPUT"
  mkdir "$TEST_DIREC/OUTPUT"
  mkdir "$TEST_DIREC/SIMULATE"
fi
#SCRIPT TO GENERATE OUT AND SIM FILES OF INPUT FILES IN INPUT DIRECTORY
cd "$CURR_DIREC/$TEST_DIREC/INPUT"

for file in $(command ls); do
  RAWFILENAME=$(basename "$file" .txt)
  cd "$CURR_DIREC/$TEST_DIREC/OUTPUT" && touch "${RAWFILENAME}_out.txt"
  cd "$CURR_DIREC/$TEST_DIREC/SIMULATE" && touch "${RAWFILENAME}_sim.txt"
  
done

