OBJECTDIR?=target/objects

KMOD=hello
OBJS=$(OBJECTDIR)/*.o
SRCS=hello.c

.include <bsd.kmod.mk>
