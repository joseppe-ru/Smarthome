#@echo off
set HOST=69.42.0.101
set USER=pi
set PASS=raspberry
echo raspberry|CLIP

cd src
scp -r smarthome %USER%@%HOST%:/home/pi/src

timeout 10

exit
plink -ssh %USER%@%HOST% -pw %PASS% "scp test.txt /home/pi/test"
sshpass -p 'raspberry' scp test.txt pi@69.42.0.101:/home/pi/test