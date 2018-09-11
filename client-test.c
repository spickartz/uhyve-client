#include <stdio.h>
#include <stdbool.h>

extern unsigned short start_app(char *);
extern unsigned short migrate(char *, char *, char *, unsigned char, unsigned char);

int main() {
	unsigned short ret = migrate("epyc-0", "live", "incremental", true, true);

	return 0;
}
