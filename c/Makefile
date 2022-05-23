CC = cc
CFLAGS = -g -O0 -Wall -Werror

mysql_LIBS := $(shell mysql_config --libs_r)
mysql_FLAGS := $(shell mysql_config --cflags)

LDLIBS += $(mysql_LIBS)
CFLAGS += $(mysql_FLAGS)

OS := $(shell uname -s)
ifeq ($(OS),Darwin)
	openssl_PREFIX := $(shell brew --prefix openssl)
	CFLAGS += -I$(openssl_PREFIX)/include
	LDLIBS += -L$(openssl_PREFIX)/lib
endif

TARGET = mysql

all: $(TARGET)

$(TARGET): $(TARGET).c
	$(CC) $(CFLAGS) -o $(TARGET) $(TARGET).c $(LDLIBS)

clean:
	$(RM) $(TARGET)
