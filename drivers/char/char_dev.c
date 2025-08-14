#include <sys/param.h>
#include <sys/module.h>
#include <sys/kernel.h>
#include <sys/systm.h>

extern struct moduledata char_mod;

DECLARE_MODULE(kld_char, char_mod, SI_SUB_DRIVERS, SI_ORDER_MIDDLE);
