CC = cc
CFLAGS = -std=c17 -O3 -Wall

bench: images libwrapper.a
	cargo bench

qoi-header:
	mkdir -p bindings
	wget https://raw.githubusercontent.com/phoboslab/qoi/master/qoi.h -O bindings/qoi.h

wrapper.o: qoi-header
	mkdir -p bindings/bin
	$(CC) $(CFLAGS) -c -o bindings/bin/wrapper.o bindings/wrapper.c

libwrapper.a: wrapper.o
	ar rcs bindings/bin/libwrapper.a bindings/bin/wrapper.o

images:
	mkdir -p images
	wget https://qoiformat.org/qoi_test_images.zip -O /tmp/qoi_test_images.zip
	unzip /tmp/qoi_test_images.zip -d images
	rm images/qoi_test_images/*.png
	mv images/qoi_test_images/* images
	rmdir images/qoi_test_images
	rm /tmp/qoi_test_images.zip

clean:
	rm -rf images
	rm -rf bindings/bin
	rm -f bindings/qoi.h
