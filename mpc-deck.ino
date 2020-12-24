#include <HID-Project.h>

typedef struct {
    int port;
    uint16_t key;
} Button;

Button buttons[] = {
    {.port = 2, .key = MEDIA_PLAY_PAUSE},
    {.port = 3, .key = MEDIA_NEXT},
    {.port = 4, .key = MEDIA_VOLUME_UP},
    {.port = 5, .key = MEDIA_VOLUME_DOWN},
};

int n_buttons = sizeof(buttons) / sizeof(*buttons);

void setup() {
    Consumer.begin();

    for (int i = 0; i < n_buttons; i++) {
        pinMode(buttons[i].port, INPUT_PULLUP);
    }
}

void loop() {
    for (int i = 0; i < n_buttons; i++) {
        if (!digitalRead(buttons[i].port)) {
            Consumer.write(buttons[i].key);
            delay(300);
        }
    }
}
