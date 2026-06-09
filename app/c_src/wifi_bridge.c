#include <app/wifi_settings.h>

void * wifi_bridge_get_ssid(void) {
    return WIFI_SSID;
}

void * wifi_bridge_get_psk(void) {
    return WIFI_PSK;
}
