#ifdef _WIN32
#define _CRT_SECURE_NO_WARNINGS
#endif

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "monocypher.h"
#include "monocypher.c"

#define MegaBytes(x) 	((x) * 1024 * 1024)
#define KiloBytes(x)    ((x) * 1024)

#define CASSETTE_MAGIC_BE   0xca55e77e
#define CASSETTE_MAGIC_LE   0x7ee755ca

typedef uint8_t CassetteObjectType;

enum
{
	CassetteObject_File,
	CassetteObject_Index,
	CassetteObject_Metadata,
	CassetteObject_Unknown = 0xff
};


struct CassetteHeader {
	uint32_t magic; 	/* @see CASSETTE_MAGIC */
	uint32_t version;
	uint16_t page_size;

	uint64_t object_count;
	uint64_t metadata_bytes;
	uint64_t first_object;

	uint8_t content_hash[64];
};

struct CassetteObjectHeader {
	uint8_t             id[64];
	uint32_t            page_count;
	CassetteObjectType  type;
	uint8_t 	    reserved[3];
};


int
CassetteHashFile(const char *path, uint8_t* hash)
{
	size_t bytes_read;
	crypto_blake2b_ctx ctx;
	uint8_t buffer[KiloBytes(64)];
	FILE *f = NULL;

	f = fopen(path, "rb");

	if (f == NULL) {
		return 1;
	}

	crypto_blake2b_init(&ctx);

	do {
		bytes_read = fread(buffer, 1, KiloBytes(64), f);
		crypto_blake2b_update(&ctx, buffer, bytes_read);
	} while (bytes_read == KiloBytes(64));

	crypto_blake2b_final(&ctx, hash);
	fclose(f);
	return 0;
}

void
PrintHash(uint8_t *hash)
{

	for(int i = 0; i < 64; ++i) {
		printf("%02x", hash[i]);
	}
	printf("\n");
}

int
main(int argc, char **argv)
{
	int files = argc;
	puts("Cassette v1.0");
	while (files --> 1) {
		uint8_t hash[64];
		CassetteHashFile(argv[files], hash);
		PrintHash(hash);
	}
	return 0;
}
