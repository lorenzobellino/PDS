ls
ls root/
ls src/kern/
sudo apt update
sudo apt upgrade
ls
cd src/
ls
cd kern/
ls
cd compile/
ls
cd DUMBVM/
ls
cd ..
rm -rf DUMBVM/
ls
cd ..
cd conf/
ls
./config DUMBVM
cd ../compile/
ls
cd DUMBVM/
ls
bmake depend
ls
bmake install
ls
bmake
bmake install
cd ..
ls
cd ..
cd root/
ls
os161 kernel
sys161 kernel
exit
ls
cd src/kern/
ls
cd conf/
ls
cd ..
cd root/
ls
cd ..
ls
cd src/
ls
cd  kern/
ls
cd main/
ls
nano hello.c
sudo apt install nano
ls
nano hello.c
code hello.c 
code ../include/hello.h
ls
code main.c 
cd ..
cd conf/
ls
cp DUMBVM HELLO
code HELLO 
ls
./config HELLO 
ls
code conf.kern 
./config HELLO 
cd ..
cd compile/
ls
cd HELLO/
ls
bmake depend
bmake 
cd ..
cd conf/
ls
./config HELLO 
cd ../compile/
ls
cd HELLO/
ls
bmake clean
bmake depend
bmake
bmake install
cd ../../
ls
cd ..
cd root/
ls
sys161 kernel
exit
cd src/kern/compile/
bmake clean
ls
cd HELLO/
ls
rm -rf *
bmake depend
bmake
bmake install
ls
cat hello.o 
ls
cat opt-hello.h 
ls
rm -rf *
bmake depend
bmake
cd src/
ls
cd configure 
cd kern/conf/
ls
./config HELLO 
cd root/
ls
file kernel
ll
sys161 kernel
