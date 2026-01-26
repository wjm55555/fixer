#include "md5.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>

int main() {
  char buf[1024];

  fgets(buf, sizeof(buf), stdin);
  buf[strcspn(buf, "\n")] = '\0';

  uint8_t out[16];
  md5((const uint8_t *)buf, strlen(buf), out);

  for (int i = 0; i < 16; i++)
    printf("%02x", out[i]);
  printf("\n");

  return 0;
}
