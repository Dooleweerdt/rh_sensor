#include <app/wifi_settings.h>

#ifdef WIFI_SSID
void * wifi_bridge_get_ssid(void) {
    return WIFI_SSID;
}
#endif

#ifdef WIFI_PSK
void * wifi_bridge_get_psk(void) {
    return WIFI_PSK;
}
#endif
