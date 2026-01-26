#include "md5.h"

#include <string.h>
#include <stdlib.h>
#include <stdint.h>
#include <stddef.h>

static uint8_t *md5_pad(const uint8_t *msg, size_t len, size_t *new_len) {
  size_t padded_len = len + 1;
  while (padded_len % 64 != 56) {
    padded_len++;
  }

  padded_len += 8;

  uint8_t *padded = calloc(padded_len, 1);
  memcpy(padded, msg, len);
  padded[len] = 0x80;

  uint64_t bit_len = len * 8;
  memcpy(padded + padded_len - 8, &bit_len, 8);

  *new_len = padded_len;
  return padded;
}

void md5(const uint8_t *initial_msg, size_t len, uint8_t digest[16]) {
  size_t new_len;
  uint8_t *msg = md5_pad(initial_msg, len, &new_len);

  uint32_t A = 0x67452301;
  uint32_t B = 0xefcdab89;
  uint32_t C = 0x98badcfe;
  uint32_t D = 0x10325476;

  for (size_t offset = 0; offset < new_len; offset += 64) {
    uint32_t *w = (uint32_t *)(msg + offset);
    uint32_t a = A, b = B, c = C, d = D;

    for (int i = 0; i < 64; i++) {
      uint32_t f, g;

      if (i < 16) {
        f = (b & c) | (~b & d);
        g = i;
      } else if (i < 32) {
        f = (d & b) | (~d & c);
        g = (5 * i + 1) % 16;
      } else if (i < 48) {
        f = b ^ c ^ d;
        g = (3 * i + 5) % 16;
      } else {
        f = c ^ (b | ~d);
        g = (7 * i) % 16;
      }

      uint32_t temp = d;
      d = c;
      c = b;
      b = b + LEFTROTATE((a + f + K[i] + w[g]), s[i]);
      a = temp;
    }

    A += a;
    B += b;
    C += c;
    D += d;
  }

  free(msg);

  memcpy(digest + 0, &A, 4);
  memcpy(digest + 4, &B, 4);
  memcpy(digest + 8, &C, 4);
  memcpy(digest + 12, &D, 4);
}

