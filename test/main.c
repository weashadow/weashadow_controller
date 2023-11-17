#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <assert.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <linux/fb.h>

#define SCREEN_X_RES 480
#define SCREEN_Y_RES 800

void fill_screen(uint32_t *buf, uint32_t color) {
    for (int i = 0; i < SCREEN_X_RES * SCREEN_Y_RES; i++) {
        buf[i] = color;
    }
}

int main() {
    int fb = open("/dev/fb0", O_RDWR);
    assert(fb > 0);

    size_t len = 4 * SCREEN_X_RES * SCREEN_Y_RES;
    uint32_t *buf = mmap(NULL, len, PROT_READ | PROT_WRITE, MAP_SHARED, fb, 0);
    assert(buf != MAP_FAILED);

    // Fill the screen with a red color
    fill_screen(buf, 0xFFFF00FF);

    munmap(buf, len);
    close(fb);

    return 0;
}
