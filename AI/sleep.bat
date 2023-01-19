@echo off


if %time:~0,2% geq 20 goto :hibernate


:sleep
shutdown -s -t 0
exit

:hibernate
shutdown -h -t 0
exit