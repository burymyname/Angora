/*
  Test:
  Nested `if` conditional statements.
  It is difficult for other fuzzers, but it is easy for Angora.
*/
#include "stdint.h"
#include "stdio.h"
#include "stdlib.h"
#include "string.h"

int main(int argc, char **argv) {

  if (argc < 2)
    return 0;

  FILE *fp;
  char buf[255];
  size_t ret;

  fp = fopen(argv[1], "rb");

  if (!fp) {
    printf("st err\n");
    return 0;
  }

  int len = 20;

  ret = fread(buf, sizeof *buf, len, fp);
  fclose(fp);
  if (ret < len) {
    printf("input fail \n");
    return 0;
  }

  uint16_t x = 0;
  int32_t y = 0;
  int32_t z = 0;
  uint32_t a = 0;
  int16_t b = 0;
  // unsigned char c[10];
  uint16_t NumChannels = 0;
  uint16_t BlockAlign = 0;
  int bits_per_sample = 0;


  // memcpy(&x, buf + 1, 2);  // x 1 - 2
  // memcpy(&y, buf + 4, 4);  // y 4 - 7
  // memcpy(&z, buf + 10, 4); // 10 - 13
  // memcpy(&a, buf + 14, 4); // 14 - 17
  // memcpy(&b, buf + 8, 2);
  memcpy(&NumChannels, buf, 2);
  memcpy(&BlockAlign, buf+2, 2);
  memcpy(&bits_per_sample, buf+4, 4);
  // memcpy(&c1, buf+1, 1);
  // uint16_t n = c << 8 | c1;

  // if (0x20 <= c && c <= 0x7E) {
  //   if (c == '"' || c == '\\') {
  //     printf("hit\n");
  //   }
  // }

  // x = x - 0x20;
  // y = y + 0x20;

  // for (int a = 0; a < 10; a++) {
  //   int chr = c[a];
  //   if (chr >=32 || chr == '\n' || chr == '\t') {
  //     printf("hit\n");
  //   }
  // }


  // x += 100;
  // y -= 100;
  // z += 1000;
  // a -= 1000;

  if (!NumChannels || NumChannels > 256 ||
      BlockAlign / NumChannels < (bits_per_sample + 7) / 8 ||
      BlockAlign / NumChannels > 4 ||
      BlockAlign % NumChannels) {
          printf("hit\n"); // error        
      }

  // if (z == a * y) {
  //   printf("hit \n");
  // }

  // if (x - 12300 > 0 && x - 12350 < 0 && z + 100000000 < 0 &&
  //     z + 100000005 > 0 && z + 100000003 != 0 && y - 987654321 >= 0 &&
  //     y - 987654325 <= 0 && a - 123456789 == 0) {

  //   printf("hey, you hit it \n");

  //   if (x == 12320) {
  //     printf("hit 2\n");
  //   }

  //   if (z + 100000004 == 0) {
  //     printf("hit 3\n");
  //   }

  //   if (a + 123456790 == 0) {
  //     printf("hit 2\n");
  //   }

  //   // abort();
  //   /* _exit(6); */
  // }
  return 0;
}
