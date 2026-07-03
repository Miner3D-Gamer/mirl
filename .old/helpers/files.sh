echo cmd /c "for /r %F in (*.rs) do echo .%~pnxF >> paths.txt" > test.sh
test.sh
rm test.sh